use std::borrow::BorrowMut;
use actix_web::{web, Responder};
use rbatis::{executor::RbatisRef, Rbatis};

use crate::model::{Claim, Profile, ResponseData, UserFollow, UserTable};
use crate::util::error::CustomError::{InternalError};

#[actix_web::get("/celeb_{username}")]
pub async fn get_profile(path: web::Path<String>, data: web::Data<Rbatis>, claim: actix_web::Result<Claim>) -> Result<impl Responder, actix_web::Error> {
    let username = path.into_inner();
    let mut rbatis = data.get_rbatis();
    let rbatis = rbatis.borrow_mut();
    let list = UserTable::select_by_column(rbatis, "username", &username)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;
    if list.is_empty() {
        return Ok(ResponseData::new("profile", None::<Profile>));
    }
    let user = list.get(0).unwrap();
    let following = match claim {
        Err(_) => false,
        Ok(claim) => {
            //再查用户关注表
            let user_follow = UserFollow::select_follow(rbatis, claim.id, user.uid)
                .await
                .map_err(|e| InternalError { message: e.to_string() })?;

            user_follow.is_some()
        }
    };
    let profile = Profile {
        username,
        following,
        bio: user.bio.to_owned(),
        image: user.image.to_owned(),
    };

    Ok(ResponseData::new("profile", profile))
}
