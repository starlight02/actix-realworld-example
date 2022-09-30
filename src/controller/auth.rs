use std::borrow::{BorrowMut};
use actix_web::{web, Responder, HttpRequest, http::header};
use rbatis::{Rbatis, executor::RbatisRef};

use crate::model::{LoginPayload, SignUpPayload, NewUser, User, UpdateUser, ResponseData};
use crate::service;
use crate::util::{
    self,
    error::CustomError::{
        InternalError, UnauthorizedError, ValidationError
    }
};

#[post("")]
pub async fn sign_up(data: web::Data<Rbatis>, payload: web::Json<SignUpPayload>) -> impl Responder {
    let payload = payload.into_inner();
    let NewUser { email, username, password } = payload.user.to_owned();
    if email.is_empty() || username.is_empty() || password.is_empty() {
        return Err(ValidationError { message: "`email`,`username` and `password` is required".to_owned() });
    };
    let data = data.into_inner();
    let mut rbatis = data.get_rbatis();
    let rbatis = rbatis.borrow_mut();
    // 先查询邮箱是否已注册
    let user = service::select_user_by_email(rbatis, &email)
        .await.map_err(|e| InternalError { message: e.to_string() })?;

    let user = match user {
        // 未注册
        None => {
            // 查询用户名是否重复
            let is_existing = service::is_uname_already_exists(rbatis, &username)
                .await.map_err(|e| InternalError { message: e.to_string() })?;
            if is_existing {
                return Err(ValidationError { message: "Username already exists".to_owned() });
            }
            let mut user = payload.user;
            let phc = util::get_phc_string(user.password.as_str());
            user.password = phc;
            // 注册操作，插入后再查询
            service::insert_new_user(rbatis, &user)
                .await.map_err(|e| InternalError { message: e.to_string() })?;
            let user = service::select_user_by_email(rbatis, &email)
                .await.map_err(|e| InternalError { message: e.to_string() })?.unwrap();

            User {
                uid: user.uid,
                email: user.email.trim().to_owned(),
                username: user.username,
                nickname: None,
                bio: None,
                image: None,
                token: None,
            }
        }
        // 数据库有相同的邮箱记录
        Some(u) => {
            // 邮箱已被注册
            if !u.deleted {
                return Err(ValidationError { message: "Email is already registered".to_owned() });
            }
            //曾注册，后注销了账号
            let is_existing = service::is_uname_already_exists(rbatis, &username)
                .await.map_err(|e| InternalError { message: e.to_string() })?;
            if is_existing {
                return Err(ValidationError { message: "Username already exists".to_owned() });
            }
            let phc = util::get_phc_string(u.password.as_str());
            // 重新激活账号
            let user = UpdateUser {
                uid: u.uid,
                email: Some(u.email.to_owned()),
                username: Some(username.to_owned()),
                password: Some(phc),
                nickname: None,
                bio: None,
                image: None,
                deleted: Some(false),
            };
            service::update_user(rbatis, &user)
                .await.map_err(|e| InternalError { message: e.to_string() })?;

            User {
                uid: u.uid,
                email: u.email,
                username: username.to_owned(),
                nickname: None,
                bio: None,
                image: None,
                token: None,
            }
        }
    };

    Ok(ResponseData::new("user", user))
}

#[post("/login")]
pub async fn login(request: HttpRequest, data: web::Data<Rbatis>, credentials: web::Json<LoginPayload>) -> impl Responder {
    let credentials = credentials.into_inner().user;
    let email = credentials.email.trim();
    let password = credentials.password.trim();

    let origin = util::get_header_value_str(&request, header::REFERER, "");
    if email.is_empty() || password.is_empty() {
        return Err(UnauthorizedError {
            realm: origin.to_owned(),
            message: "`email` and `password` is required".to_owned(),
        });
    }
    let data = data.into_inner();
    let rbatis = data.get_rbatis();
    // let rbatis = rbatis.borrow_mut();

    let user = service::select_user_by_email(rbatis, email)
        .await.map_err(|e| InternalError { message: e.to_string() })?;
    if user.is_none() || user.is_some_and(|u| u.deleted) {
        return Err(UnauthorizedError {
            realm: origin.to_owned(),
            message: "Incorrect username or password".to_owned(),
        });
    }
    let user = user.unwrap();
    let is_verified = util::verify_password(password, &user.password);
    if !is_verified {
        return Err(UnauthorizedError {
            realm: origin.to_owned(),
            message: "Incorrect username or password".to_owned(),
        });
    }
    // 密码校验通过，签发 Token
    let token = util::sign_token(user.uid, user.email.to_owned()).unwrap();
    let user = Some(User {
        uid: user.uid,
        email: user.email,
        username: user.username,
        nickname: user.nickname,
        bio: user.bio,
        image: user.image,
        token: Some(token),
    });

    Ok(ResponseData::new("user", user))
}
