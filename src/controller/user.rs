use actix_web::{web, HttpRequest, HttpResponse, Responder, http::Version, get, post, put, delete};
use actix_web::web::Buf;
use rbatis::{Rbatis, executor::RbatisRef};
use rbs::to_value;

use crate::model::{RequestPayload, ResponseData, user::{NewUser, User}};
use crate::model::user::UserTable;
use crate::service::user::{insert_new_user, is_uname_already_exists, select_user_by_email, select_user_by_uid};
use crate::util::error::CustomError;

#[get("/{uid}")]
pub async fn get_user_info(data: web::Data<Rbatis>, path: web::Path<u32>) -> Result<HttpResponse, actix_web::Error> {
    let uid = path.into_inner();
    let rbatis = data.get_rbatis();

    let user = select_user_by_uid(rbatis, uid)
        .await
        .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
    // let data = User::select_by_column(&mut rb, field_name!(User.uid), uid).await.unwrap();
    // println!("select_all_by_id = {:?}", &user);
    Ok(HttpResponse::Ok().json(ResponseData { user }))
}

#[post("")]
pub async fn sign_up(data: web::Data<Rbatis>, payload: web::Json<RequestPayload>) -> impl Responder {
    let NewUser { email, username, password } = &payload.user;
    if email.is_empty() {
        return Err(CustomError::ValidationError { message: "`email` is required".to_owned() });
    } else if username.is_empty() {
        return Err(CustomError::ValidationError { message: "`username` is required".to_owned() });
    } else if password.is_empty() {
        return Err(CustomError::ValidationError { message: "`password` is required".to_owned() });
    };
    let rbatis = data.get_rbatis();
    // 先查询邮箱是否已注册
    let user = select_user_by_email(rbatis, email)
        .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;
    // // 再查询用户是否重复
    // let is_existing = is_uname_already_exists(rbatis, username)
    //     .await
    //     .map_err(|e| CustomError::InternalError { message: e.to_string() })?;

    match user {
        // 未注册
        None => {
            // 查询用户名是否重复
            let is_existing = is_uname_already_exists(rbatis, username)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;
            if is_existing {
                return Err(CustomError::ValidationError { message: "Username already exists".to_owned() });
            }
            // 注册操作，插入后再查询
            let rows = insert_new_user(rbatis, &payload.user)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;
            if rows == 1 {
                let user = select_user_by_email(rbatis, email)
                    .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;

                user
            }
        }
        // 数据库有相同的邮箱记录
        Some(u) => {
            // 邮箱已被注册
            if !u.deleted {
                return Err(CustomError::ValidationError { message: "Email is already registered".to_owned() });
            }
            //曾注册，后注销了账号
            let is_existing = is_uname_already_exists(rbatis, username)
                .await.map_err(|e| CustomError::InternalError { message: e.to_string() })?;
            if is_existing {
                return Err(CustomError::ValidationError { message: "Username already exists".to_owned() });
            }
            // 更新操作
        }
    }

    Ok(HttpResponse::Ok().body("s"))
}
