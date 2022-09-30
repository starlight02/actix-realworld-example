use rbatis::{Rbatis};
use rbs::to_value;
use crate::model::user::{NewUser, UpdateUser, User, UserTable, UserFollow, LoginCredentials};

impl_update!(UpdateUser {}, r#""user""#);
impl_select!(UserTable {}, r#""user""#);
impl_select!(User {select_user_by_uid(table_name:&str,uid:u32) -> Option => "`WHERE uid = #{uid} AND deleted = false`" });
impl_select!(User {select_user_by_email(table_name:&str,uname:&str) -> Option => "`WHERE username = #{uname} AND deleted = false`" });
impl_select!(UserFollow {select_follow(uid:u32, follow_uid:u32) -> Option => "`WHERE uid = #{uid} AND follow_uid = #{follow_uid}`"});
impl_insert!(UserFollow {});
impl_delete!(UserFollow {delete_follow(uid:u32, follow_uid:u32) => "`WHERE uid = #{uid} AND follow_uid = #{follow_uid}`"});

/// 通过 uid 查询正常用户
pub async fn select_user_by_uid(rb: &Rbatis, uid: u32) -> Result<Option<User>, rbatis::Error> {
    rb.fetch_decode(
        r#"SELECT * FROM "user" WHERE uid = ? AND deleted = false"#,
        vec![to_value!(uid)],
    ).await
}

/// 通过 email 查询用户
pub async fn select_user_by_email(rb: &Rbatis, email: &str) -> Result<Option<UserTable>, rbatis::Error> {
    rb.fetch_decode(
        r#"SELECT * FROM "user" WHERE email = ?"#,
        vec![to_value!(email)],
    ).await
}

/// 通过 email和 password_hash 查询用户
pub async fn select_user_by_email_and_password(rb: &Rbatis, credentials: &LoginCredentials) -> Result<Option<User>, rbatis::Error> {
    rb.fetch_decode(
        r#"SELECT * FROM "user" WHERE email = ? AND password = ? AND deleted = false;"#,
        vec![to_value!(&credentials.email), to_value!(&credentials.password)],
    ).await
}

/// 查询用户名是否已存在
pub async fn is_uname_already_exists(rb: &Rbatis, uname: &str) -> Result<bool, rbatis::Error> {
    let result: u32 = rb.fetch_decode(
        r#"SELECT count(1) FROM "user" WHERE username = ?"#,
        vec![to_value!(uname)],
    ).await?;

    Ok(result == 1)
}

// 插入一条新用户记录
pub async fn insert_new_user(rb: &Rbatis, u: &NewUser) -> Result<u64, rbatis::Error> {
    let exec_result = rb.exec(
        r#"INSERT INTO "user" (email, username, password) VALUES (?, ?, ?);"#,
        vec![to_value!(u.email.as_str()), to_value!(u.username.as_str()), to_value!(u.password.as_str())],
    ).await?;

    Ok(exec_result.rows_affected)
}

// 更新一条用户记录
pub async fn update_user(rb: &Rbatis, u: &UpdateUser) -> Result<u64, rbatis::Error> {
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
