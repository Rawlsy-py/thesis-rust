use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    country_code: String,
    points_balance: i32,
}

async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner())
}

async fn read_users() -> impl Responder {
    HttpResponse::Ok().json(vec![User {
        name: "John".to_string(),
        country_code: "US".to_string(),
        points_balance: 100,
    }])
}

async fn read_user(user_name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(User {
        name: user_name.into_inner(),
        country_code: "US".to_string(),
        points_balance: 100,
    })
}

async fn update_user(_user_name: web::Path<String>, user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner()) // Dummy response
}

async fn delete_user(user_name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Deleted user: {}", user_name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080/");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(read_users))
            .route("/users/", web::post().to(create_user))
            .route("/users/", web::get().to(read_users))
            .route("/users/{user_name}", web::get().to(read_user))
            .route("/users/{user_name}", web::put().to(update_user))
            .route("/users/{user_name}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
