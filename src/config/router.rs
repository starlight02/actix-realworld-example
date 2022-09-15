use actix_web::web::{self, ServiceConfig};
use crate::controller::user;
use crate::controller::auth;

pub fn router(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(
                web::scope("/users").service(auth::sign_up)
            )
            .service(
                web::scope("/user").service(user::get_user_info)
            )
    );
    // .service(
    //     scope("/api/v1/user")
    //         // 身份验证中间件
    //         // 不能写在 main 那里，那里会拦截全部请求
    //         // 这里对此 scope 下的所有路由起作用
    //         .wrap(HttpAuthentication::bearer(validator))
    //         .service(get_all_users)
    //         .service(get_user)
    //         .service(add_new_user)
    //         .service(update_user)
    //         .service(del_user)
    // );
    // .service(
    //     scope("/api/v1")
    //         .service(login)
    // );
}
