use std::borrow::BorrowMut;
use actix_web::{web, Responder};
use rbatis::{executor::RbatisRef, Rbatis};

use crate::model::{Claims, Profile, ResponseProfile, ResponseUser, UpdateUser, UpdateUserPayload, UserFollow, UserTable};
use crate::util::error::CustomError::{InternalError};

#[actix_web::get("/{username}")]
pub async fn get_profile(path: web::Path<String>, data: web::Data<Rbatis>, claims: Claims) -> Result<impl Responder, actix_web::Error> {
    let username = path.into_inner();
    let mut rbatis = data.get_rbatis();
    let rbatis = rbatis.borrow_mut();
    let list = UserTable::select_by_column(rbatis, "email", &username)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;
    if list.is_empty() {
        return Ok(ResponseProfile { profile: None });
    }
    let user = list.get(0).unwrap();
    //再查用户关注表
    let user_follow = UserFollow::select_follow(rbatis, claims.id, user.uid)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;

    let profile = Profile {
        username,
        bio: user.bio.to_owned(),
        image: user.images.to_owned(),
        following: user_follow.is_some(),
    };

    Ok(ResponseProfile { profile: Some(profile) })
}