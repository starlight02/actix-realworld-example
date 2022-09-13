// use actix_web::{ Error, HttpResponse, post};
// use actix_web::web::Json;
//
// use crate::util::utils::sign_token;
// use crate::model::user::{LoginCredentials, User};
// use crate::util::error::CustomError;

// #[post("/login")]
// pub async fn login(credentials: Json<LoginCredentials>) -> Result<HttpResponse, Error> {
//     let wrapper = RB.new_wrapper()
//         .eq("uname", credentials.uname.as_str())
//         .and()
//         .eq("password", credentials.password.as_str());
//     let user = RB.fetch_by_wrapper::<Option<User>>(&wrapper)
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?
//         .ok_or(CustomError::LoginError { message: "用户名或密码错误".to_string() })?;
//     // // 用户名或密码不匹配
//     // if user.uname != lc.uname || user.password != Some(lc.password.to_owned()) {
//     //     return Err(CustomError::LoginError { message: "用户名或密码错误".to_string() }.into());
//     // }
//
//     let token = sign_token(user.to_owned())?;
//
//     Ok(HttpResponse::Ok().cookie(Cookie::new("AILI.MOE", token)).finish())
// }

// pub async fn sign_up() {}
