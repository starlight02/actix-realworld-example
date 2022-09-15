use std::borrow::{BorrowMut};
use actix_web::{web, Responder, get, post, put, delete};
use rbatis::{Rbatis};

use crate::model::{
    RequestPayload, ResponseData,
    user::{NewUser, User, UpdateUser},
};
use crate::model::user::LoginCredentials;
use crate::service::user::{insert_new_user, is_uname_already_exists, select_user_by_email, update_user};
use crate::util::error::CustomError;
use crate::util::utils::get_phc_string;

#[post("")]
pub async fn sign_up(data: web::Data<Rbatis>, payload: web::Json<RequestPayload>) -> impl Responder {
    let payload = payload.into_inner();
    let NewUser { email, username, password } = payload.user.to_owned();
    if email.is_empty() {
        return Err(CustomError::ValidationError { message: "`email` is required".to_owned() });
    } else if username.is_empty() {
        return Err(CustomError::ValidationError { message: "`username` is required".to_owned() });
    } else if password.is_empty() {
        return Err(CustomError::ValidationError { message: "`password` is required".to_owned() });
    };
    let mut rbatis = data.get_ref();
    let rbatis = rbatis.borrow_mut();
    // 先查询邮箱是否已注册
    let user = select_user_by_email(rbatis, &email)
        .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;

    let user = match user {
        // 未注册
        None => {
            // 查询用户名是否重复
            let is_existing = is_uname_already_exists(rbatis, &username)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;
            if is_existing {
                return Err(CustomError::ValidationError { message: "Username already exists".to_owned() });
            }
            let mut user = payload.user;
            let phc = get_phc_string(user.password.as_str());
            user.password = phc;
            // 注册操作，插入后再查询
            insert_new_user(rbatis, &user)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;
            let user = select_user_by_email(rbatis, &email)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?.unwrap();

            User {
                uid: user.uid,
                email: user.email.trim().to_owned(),
                username: user.username,
                nickname: None,
                bio: None,
                images: None,
                token: None,
            }
        }
        // 数据库有相同的邮箱记录
        Some(u) => {
            // 邮箱已被注册
            if !u.deleted {
                return Err(CustomError::ValidationError { message: "Email is already registered".to_owned() });
            }
            //曾注册，后注销了账号
            let is_existing = is_uname_already_exists(rbatis, &username)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;
            if is_existing {
                return Err(CustomError::ValidationError { message: "Username already exists".to_owned() });
            }
            let phc = get_phc_string(u.password.as_str());
            // 重新激活账号
            let user = UpdateUser {
                uid: u.uid,
                email: Some(u.email.to_owned()),
                username: Some(username.to_owned()),
                password: Some(phc),
                nickname: Some("".to_owned()),
                bio: Some("".to_owned()),
                image: Some("".to_owned()),
                deleted: Some(false),
            };
            update_user(rbatis, &user)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;

            User {
                uid: u.uid,
                email: u.email,
                username: username.to_owned(),
                nickname: None,
                bio: None,
                images: None,
                token: None,
            }
        }
    };

    Ok(ResponseData { user: Some(user) })
}

#[post("/login")]
pub async fn login(data: web::Data<Rbatis>, credentials: web::Json<LoginCredentials>) -> impl Responder {
    let credentials = credentials.into_inner();
    let email = credentials.email.trim();
    let password = credentials.password.trim();


    ""
}
