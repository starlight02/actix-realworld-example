use rbatis::{Error, Rbatis};
use rbs::to_value;
use crate::model::user::{NewUser, UpdateUser, User, UserTable};

/// 通过 uid 查询正常用户
pub async fn select_user_by_uid(rb: &Rbatis, uid: u32) -> Result<Option<User>, Error> {
    rb.fetch_decode(
        r#"SELECT * FROM "user" WHERE uid = ? AND deleted = false"#,
        vec![to_value!(uid)],
    ).await
}

/// 通过 email 查询用户
pub async fn select_user_by_email(rb: &Rbatis, email: &str) -> Result<Option<UserTable>, Error> {
    rb.fetch_decode(
        r#"SELECT * FROM "user" WHERE email = ?"#,
        vec![to_value!(email)],
    ).await
}

/// 查询用户名是否已存在
pub async fn is_uname_already_exists(rb: &Rbatis, uname: &str) -> Result<bool, Error> {
    let result: u32 = rb.fetch_decode(
        r#"SELECT count(1) FROM "user" WHERE username = ?"#,
        vec![to_value!(uname)],
    ).await?;

    Ok(result == 1)
}

// 插入一条新用户记录
pub async fn insert_new_user(rb: &Rbatis, u: &NewUser) -> Result<u64, Error> {
    let exec_result = rb.exec(
        r#"INSERT INTO "user" (email, username, password) VALUES (?, ?, ?);"#,
        vec![to_value!(u.email.as_str()), to_value!(u.username.as_str()), to_value!(u.password.as_str())],
    ).await?;

    Ok(exec_result.rows_affected)
}

// 更新一条用户记录
pub async fn update_user(rb: &Rbatis, u: &UpdateUser) -> Result<u64, Error> {
    let exec_result = rb.exec(
        r#"
        UPDATE "user"
        SET email = ?, username = ?, password = ?, nickname = ?, bio =?, image = ?, deleted = ?
        WHERE uid = ?;
        "#,
        vec![
            to_value!(&u.email),
            to_value!(&u.username),
            to_value!(&u.password),
            to_value!(&u.nickname),
            to_value!(&u.bio),
            to_value!(&u.image),
            to_value!(u.deleted),
            to_value!(u.uid),
        ],
    ).await?;

    Ok(exec_result.rows_affected)
}
// use actix_web::{
//     Either,
//     get, post, put, delete,
//     HttpRequest, HttpResponse, Responder, web,
//     http::Version
// };
// use serde::{Deserialize, Serialize};
// // use crate::util::error::CustomError;
//
// #[derive(Deserialize, Serialize)]
// pub struct Info {
//     name: String,
// }
//
// #[get("/")]
// pub async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello Actix-Web!")
// }
//
// // 请求必须带有 name 的 query 参数才会正确响应
// #[get("/info")]
// pub async fn get_info(info: web::Query<Info>) -> impl Responder {
//     HttpResponse::Ok().body(format!("Hello {}", info.name))
// }
//
// // 要返回不同类型可以使用 Either
// #[get("/api/info")]
// pub async fn get_api_info(req: HttpRequest) -> Either<HttpResponse, Result<HttpResponse, actix_web::Error>> {
//     let is_not_http_11 = req.head().version != Version::HTTP_11;
//     if is_not_http_11 {
//         return Either::Left(HttpResponse::BadRequest().finish());
//     }
//     Either::Right(Ok(HttpResponse::Ok().body("这是 v1 版本的 api！")))
// }
//
//
// // 必须有路由参数 user_id 和 friend 且类型正确才会正确响应
// // 多个路由参数使用元组类型
// #[get("/user/{user_id}/{friend}")]
// pub async fn get_user_info(path: web::Path<(u32, String)>) -> impl Responder {
//     let (user_id, friend) = path.into_inner();
//     HttpResponse::Ok().body(format!("Welcome {}, user_id {}!", friend, user_id))
// }
//
// /// 使用 serde 提取表单数据
// /// 仅当 content type 类型是  *x-www-form-urlencoded* 是 handler 处理函数才会被调用
// /// 且请求中的内容能够被反序列化到一个 "Info" 结构体中去.
// #[post("/user")]
// pub async fn form(form: web::Form<Info>) -> actix_web::Result<String> {
//     Ok(format!("Welcome {}!", form.name))
// }

// #[get("/app/state")]
// pub async fn state(data: web::Data<ApplicationState>) -> Result<HttpResponse, actix_web::Error> {
//     let result = data.rbatis.driver_type().unwrap().to_owned();
//
//     Ok(HttpResponse::Ok().body(result))
// }

// #[get("/all")]
// pub async fn get_all_users(req: HttpRequest) -> Result<HttpResponse, Error> {
//     debug!("query string: {}", req.query_string());
//     let query_map = from_str::<HashMap<String, String>>(req.query_string()).unwrap();
//     let page = query_map
//         .get("page")
//         .unwrap_or(&String::from("1"))
//         .parse::<u64>()
//         .map_err(|_e| CustomError::ValidationError { message: "\"page\"参数必须是数字".to_string() })?;
//     let page_size = query_map
//         .get("page_size")
//         .unwrap_or(&String::from("20"))
//         .parse::<u64>()
//         .map_err(|_e| CustomError::ValidationError { message: "\"page_size\"参数必须是数字".to_string() })?;
//
//     let request = PageRequest::new(page, page_size);
//     let wrapper = RB.new_wrapper();
//     let result = RB.fetch_page_by_wrapper::<User>( &wrapper, &request)
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//
//     // 不分页
//     // let result = RB.list::<User>("")
//     //     .await
//     //     .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//
//     Ok(HttpResponse::Ok().json(result))
// }

// #[get("/{id}")]
// pub async fn get_user(Path(id): Path<u32>) -> Result<HttpResponse, Error> {
//     //  wrapper 写法
//     // let wrapper = RB.new_wrapper().eq("uid", id).unwrap();
//     // let user = RB.fetch_by_wrapper::<Option<User>>("", &wrapper)
//     //     .await
//     //     .map_err(|e| error::ErrorInternalServerError(e))?;
//
//     let user = RB.fetch_by_column::<Option<User>, String>("uid", &id.to_string())
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//
//     debug!("{:?}", user);
//     Ok(HttpResponse::Ok().json(user))
// }

// #[post("")]
// pub async fn add_new_user(user: Json<NewUser>) -> Result<HttpResponse, Error> {
//     debug!("{:?}", user.0);
//
//     let value = query_uname_repeat(&RB, &user.0.uname)
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//
//     let empty_list = Vec::new();
//     let value = value.as_array().unwrap_or(&empty_list);
//     if !value.is_empty() {
//         return Err(CustomError::ValidationError { message: "用户名已存在".to_string() }.into());
//     }
//
//     let result = RB.save( &user.0,&[])
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//
//     let id = result.last_insert_id.unwrap() as u32;
//     let new_user = RB.fetch_by_column::<Option<User>, String>("uid", &id.to_string())
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//
//     Ok(HttpResponse::Created().json(new_user))
// }


// #[put("/{id}")]
// pub async fn update_user(Path(id): Path<u32>, user: Json<UpdateUser>) -> Result<HttpResponse, Error> {
//     if !id.eq(&user.0.uid) {
//         return Err(CustomError::ValidationError { message: "\"uid\"不一致".to_string() }.into());
//     }
//
//     let empty_list = Vec::new();
//
//     let result = query_uid_repeat(&RB, &id)
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//     let list = result.as_array().unwrap_or(&empty_list);
//     if list.is_empty() {
//         return Err(CustomError::ValidationError { message: "此用户不存在".to_string() }.into());
//     }
//
//     let result = query_uname_repeat(&RB, &user.0.uname)
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//     let list = result.as_array().unwrap_or(&empty_list);
//     if !list.is_empty() {
//         return Err(CustomError::ValidationError { message: "用户名已存在".to_string() }.into());
//     }
//
//     let mut update_user = user.clone();
//     let row = RB.update_by_column::<UpdateUser>("uid", &mut update_user)
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//
//     if row > 0 {
//         Ok(HttpResponse::Ok().finish())
//     } else {
//         Ok(HttpResponse::NotModified().finish())
//     }
// }

// #[delete("/{id}")]
// pub async fn del_user(Path(id): Path<u32>) -> Result<HttpResponse, Error> {
//     let empty_list = Vec::new();
//
//     let result = query_uid_repeat(&RB, &id)
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//     let list = result.as_array().unwrap_or(&empty_list);
//     if list.is_empty() {
//         return Err(CustomError::ValidationError { message: "此用户不存在".to_string() }.into());
//     }
//
//     let row = RB.remove_by_column::<User, String>("", &id.to_string())
//         .await
//         .map_err(|e| CustomError::InternalError { message: e.to_string() })?;
//
//     if row > 0 {
//         Ok(HttpResponse::Ok().finish())
//     } else {
//         Ok(HttpResponse::NotModified().finish())
//     }
// }

//
// #[sql(rb, "SELECT count(1) as count FROM user WHERE uname = ? AND del = 0 having count > 0")]
// async fn query_uname_repeat(rb: &Rbatis, name: &str) -> Result<Value, RError> {}
//
// #[sql(rb, "SELECT count(1) as count FROM user WHERE uid = ? AND del = 0 having count > 0")]
// async fn query_uid_repeat(rb: &Rbatis, id: &u32) -> Result<Value, RError> {}


// #[post("/payload")]
// pub async fn payload(mut body: web::Payload) -> Result<HttpResponse, Error> {
//     let mut bytes = web::BytesMut::new();
//     while let Some(item) = body.next().await {
//         bytes.extend_from_slice(&item?);
//     }
//
//     Ok(HttpResponse::Ok().body(format!("Body {}!", String::from_utf8(bytes.to_vec()).unwrap())))
// }
