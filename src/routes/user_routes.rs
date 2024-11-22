use actix_web::web;
use crate::controllers::user_controller;

#[allow(dead_code)]
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("/get", web::get().to(user_controller::get_users))
            .route("/info", web::get().to(user_controller::index))
            // .route("", web::get().to(user_controller::get_users))
            // .route("/{id}", web::get().to(user_controller::get_user))
            // .route("", web::post().to(user_controller::create_user))
            // .route("/{id}", web::put().to(user_controller::update_user))
            // .route("/{id}", web::delete().to(user_controller::delete_user))
    );
} 