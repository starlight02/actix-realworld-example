use std::io;
use actix_cors::Cors;
use actix_web::{
    web, App, HttpServer,
    middleware::{DefaultHeaders, Logger, ErrorHandlers},
    http::{header, StatusCode},
    middleware::{Compress},
};
#[cfg(debug_assertions)]
use log::info;
use actix_realworld_example::{app_middleware, database, app_log, router, CONFIG};

#[tokio::main]
async fn main() -> io::Result<()> {
    // 日志初始化
    app_log::init_logger();
    // app 状态初始化
    let data = web::Data::new(database::init_pool());

    #[cfg(debug_assertions)]
    {
        let local_ip = local_ipaddress::get().unwrap();
        info!("Actix-web App Running :");
        info!("Local:    http://localhost:{}", CONFIG.PORT);
        info!("Network:  http://{}:{}", local_ip, CONFIG.PORT);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            // 日志中间件
            .wrap(Logger::default())
            // 默认响应的头部的中间件
            .wrap(DefaultHeaders::new()
                .add(("X-Server-Version", "0.1"))
                .add(header::ContentType::json())
            )
            // CORS 中间件
            .wrap(Cors::permissive())
            // 错误处理中间件
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::BAD_REQUEST, app_middleware::format_response)
            )
            .wrap(Compress::default())
            // 配置路由
            .configure(router::router)
    })
        .bind(format!("{}:{}", CONFIG.BIND_HOST, CONFIG.PORT))?
        .run()
        .await
}
