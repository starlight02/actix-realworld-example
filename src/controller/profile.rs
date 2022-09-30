use std::borrow::BorrowMut;
use actix_web::{web, Responder};
use rbatis::{executor::RbatisRef, Rbatis};

use crate::model::{Claim, Profile, ResponseData, User, UserFollow};
use crate::util::error::CustomError::{InternalError, ValidationError};

pub async fn get_profile(path: web::Path<String>, data: web::Data<Rbatis>, claim: actix_web::Result<Claim>) -> actix_web::Result<impl Responder> {
    let username = path.into_inner();
    let mut rbatis = data.get_rbatis();
    let rbatis = rbatis.borrow_mut();
    let user = User::select_user_by_email(rbatis, r#""user""#, &username)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;
    if user.is_none() {
        return Ok(ResponseData::new("profile", None::<Profile>));
    }
    let user = user.unwrap();
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

#[post("/celeb_{username}/follow")]
pub async fn follow_user(path: web::Path<String>, data: web::Data<Rbatis>, claim: Claim) -> impl Responder {
    let username = path.into_inner();
    let mut rbatis = data.get_rbatis();
    let rbatis = rbatis.borrow_mut();

    let user = User::select_user_by_email(rbatis, r#""user""#, &username)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;
    if user.is_none() {
        return Err(ValidationError { message: "The user does not exist".to_owned() });
    }

    let User { uid, bio, image, .. } = user.unwrap();
    let user_follow = UserFollow { uid: claim.id, follow_uid: uid };
    let result = UserFollow::insert(rbatis, &user_follow)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;
    if result.rows_affected < 1 {
        return Err(InternalError { message: "Follow may not be successful".to_owned() });
    }
    let profile = Profile {
        username,
        bio,
        image,
        following: true,
    };

    Ok(ResponseData::same(profile))
}

// #[routes]
#[delete("/celeb_{username}/follow")]
pub async fn unfollow_user(path: web::Path<String>, data: web::Data<Rbatis>, claim: Claim) -> impl Responder {
    let username = path.into_inner();
    let mut rbatis = data.get_rbatis();
    let rbatis = rbatis.borrow_mut();

    let user = User::select_user_by_email(rbatis, r#""user""#, &username)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;
    if user.is_none() {
        return Err(ValidationError { message: "The user does not exist".to_owned() });
    }
    let User { uid, bio, image, .. } = user.unwrap();
    let result = UserFollow::delete_follow(rbatis, claim.id, uid)
        .await
        .map_err(|e| InternalError { message: e.to_string() })?;
    if result.rows_affected < 1 {
        return Err(ValidationError { message: "Not following the user".to_owned() });
    }
    let profile = Profile {
        username,
        bio,
        image,
        following: true,
    };

    Ok(ResponseData::same(profile))
}