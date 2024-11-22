use actix_web::{web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::models::user::User;

pub async fn get_users(pool: web::Data<MySqlPool>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Failed to fetch users: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
    
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, My Api!")
} 