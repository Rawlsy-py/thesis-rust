use crate::AppState;
use actix_web::{get, web::Data, HttpResponse, Responder};
use log::error;
use serde::Serialize;
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow)]
struct Users {
    id: i32,
    name: String,
    email: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Users>("SELECT * FROM USERS;")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => {
            if users.is_empty() {
                HttpResponse::NotFound().json("No users found")
            } else {
                HttpResponse::Ok().json(users)
            }
        }
        Err(e) => {
            error!("Database query failed: {:?}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}
