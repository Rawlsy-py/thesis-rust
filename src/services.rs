use crate::AppState;
use actix_web::{get, web::Data, HttpResponse, Responder};
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
    // "GET /users".to_string()

    match sqlx::query_as::<_, Users>("SELECT * FROM USERS;")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}
