use std::borrow::{BorrowMut};
use actix_web::{web, HttpRequest, HttpResponse, Responder, http::Version, get, post, put, delete};
use rbatis::{Rbatis, executor::RbatisRef};

use crate::model::{
    RequestPayload, ResponseData,
    user::{NewUser, User, UpdateUser},
};
use crate::service::user::{insert_new_user, is_uname_already_exists, select_user_by_email, select_user_by_uid, update_user};
use crate::util::error::CustomError;

#[get("/{uid}")]
pub async fn get_user_info(data: web::Data<Rbatis>, path: web::Path<u32>) -> Result<impl Responder, actix_web::Error> {
    let uid = path.into_inner();
    let rbatis = data.get_rbatis();

    let user = select_user_by_uid(rbatis, uid)
        .await
        .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
    // let data = User::select_by_column(&mut rb, field_name!(User.uid), uid).await.unwrap();
    // println!("select_all_by_id = {:?}", &user);
    Ok(ResponseData { user })
}

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
            // 注册操作，插入后再查询
            insert_new_user(rbatis, &payload.user)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;
            let user = select_user_by_email(rbatis, &email)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?.unwrap();

            User {
                uid: user.uid,
                email: user.email,
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
            // 重新激活账号
            let user = UpdateUser {
                uid: u.uid,
                email: Some(u.email.to_string()),
                username: Some(username.to_owned()),
                password: Some(password.to_owned()),
                nickname: Some("".to_owned()),
                bio: Some("".to_owned()),
                images: Some("".to_owned()),
                deleted: Some(false),
            };
            let result = update_user(rbatis, &user)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;
            println!("update result {:#?}", result);

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
