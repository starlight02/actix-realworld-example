use std::borrow::BorrowMut;
use actix_web::{web, Responder};
use rbatis::{executor::RbatisRef, Rbatis};

use crate::model::{Claim, RealWorldToken, ResponseUser, UpdateUser, UpdateUserPayload};
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
    Ok(ResponseUser { user })
}

#[actix_web::get("")]
pub async fn get_current_user(data: web::Data<Rbatis>, token: RealWorldToken, claims: Claim) -> Result<impl Responder, actix_web::Error> {
    let data = data.into_inner();
    let rbatis = data.get_rbatis();
    let mut user = service::select_user_by_uid(rbatis, claims.id)
        .await.map_err(|e| InternalError { message: e.to_string() })?.unwrap();
    user.token = token.token;

    Ok(ResponseUser { user: Some(user) })
}

#[actix_web::put("")]
pub async fn update_user(data: web::Data<Rbatis>, user: web::Json<UpdateUserPayload>, claims: Claim) -> impl Responder {
    let data = data.into_inner();
    let mut rbatis = data.get_rbatis();
    let rbatis = rbatis.borrow_mut();
    let mut user = user.into_inner().user;
    user.uid = claims.id;

    let result = UpdateUser::update_by_column(rbatis, &user, "uid")
        .await.map_err(|e| InternalError { message: e.to_string() })?;
    if result.rows_affected < 1 {
        return Err(InternalError { message: "Update may not be successful".to_owned() });
    }
    let user = service::select_user_by_uid(rbatis, claims.id)
        .await.map_err(|e| InternalError { message: e.to_string() })?.unwrap();

    Ok(ResponseUser { user: Some(user) })
}
