use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;
use services::fetch_users;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection_string =
        std::env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .expect("Error building a connection pool");

    println!("Server running at http://127.0.0.1:8080/users");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(fetch_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
