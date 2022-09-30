use actix_web::web::{self, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::controller::{user, auth, profile};
use crate::middleware;

pub fn router(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(
                web::scope("/users")
                    .service(auth::sign_up)
                    .service(auth::login)
            )
            .service(
                web::scope("/user")
                    .wrap(HttpAuthentication::with_fn(middleware::auth::validator))
                    .service(user::get_user_info)
                    .service(user::get_current_user)
                    .service(user::update_user)
            )
            // FIXME: Same prefix but no authentication required
            .service(
                web::resource("/profiles/celeb_{username}")
                    .route(web::get().to(profile::get_profile))
            )
            .service(
                web::scope("/profiles")
                    .wrap(HttpAuthentication::with_fn(middleware::auth::validator))
                    .service(profile::follow_user)
                    .service(profile::unfollow_user)
            )
    );
}
