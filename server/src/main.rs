use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod models;

mod handlers;
use handlers::{create_user, get_users, get_user, delete_user, get_posts, get_post, get_user_posts, create_user_post, delete_post};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set");
    let db_user = std::env::var("DB_USER").expect("DB_USER must be set");
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_port = std::env::var("DB_PORT").expect("DB_PORT must be set");
    let db_host = std::env::var("DB_HOST").expect("DB_HOST must be set");
    let db_url = format!(
        "postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"
    );

    println!("Change was made!!!");
    println!("Establishing connection pool at {}:{}...", &db_host, &db_port);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error establishing connection pool.");

    println!("Successfully created connection pool.");
    println!("Starting server.");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(create_user)
            .service(get_users)
            .service(get_user)
            .service(delete_user)
            .service(get_posts)
            .service(get_post)
            .service(get_user_posts)
            .service(create_user_post)
            .service(delete_post)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
