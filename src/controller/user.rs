use actix_web::{web, Responder, get};
use rbatis::{Rbatis, executor::RbatisRef};

use crate::model::{ResponseData};
use crate::service::user::{select_user_by_uid};
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
