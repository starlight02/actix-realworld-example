use actix_web::{web, Responder};
use rbatis::{Rbatis, executor::RbatisRef};

use crate::model::{Claims, RealWorldToken, ResponseData};
use crate::service;
use crate::util::error::CustomError::InternalError;

#[actix_web::get("/{uid}")]
pub async fn get_user_info(data: web::Data<Rbatis>, path: web::Path<u32>) -> Result<impl Responder, actix_web::Error> {
    let uid = path.into_inner();
    let rbatis = data.get_rbatis();

    let user = service::select_user_by_uid(rbatis, uid)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;
    // let data = User::select_by_column(&mut rb, field_name!(User.uid), uid).await.unwrap();
    // println!("select_all_by_id = {:?}", &user);
    Ok(ResponseData { user })
}

#[actix_web::get("")]
pub async fn get_current_user(data: web::Data<Rbatis>, token: RealWorldToken, claims: Claims) -> Result<impl Responder, actix_web::Error> {
    let data = data.into_inner();
    let rbatis = data.get_rbatis();
    let mut user = service::select_user_by_uid(rbatis, claims.id)
        .await.map_err(|e| InternalError { message: e.to_string() })?.unwrap();
    user.token = Some(token.token);

    Ok(ResponseData { user: Some(user) })
}
