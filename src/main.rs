// main.rs

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::{prelude::*, PgConnection};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize, Queryable)]
struct MyModel {
    id: i32,
    country_code: String,
    balance: f64,
}


#[derive(Debug, Deserialize, Serialize)]
struct UpdateBalance {
    id: i32,
    balance: f64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Set up the database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_data))
            .route("/update-balance", web::post().to(update_balance))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

async fn get_data() -> impl Responder {
    let conn = establish_connection();
    let data = my_table::table
        .limit(10)
        .load::<MyModel>(&conn)
        .expect("Error loading data");
    HttpResponse::Ok().json(data)
}

async fn update_balance(balance_info: web::Json<UpdateBalance>) -> impl Responder {
    let conn = establish_connection();

    diesel::update(my_table::table.find(balance_info.id))
        .set(my_table::balance.eq(balance_info.balance))
        .execute(&conn)
        .expect("Error updating balance");

    HttpResponse::Ok().json("Balance updated successfully")
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
