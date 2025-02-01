use crate::config::Config;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};

mod config;
mod routes;
mod services;
mod models;
mod schema;
mod tests;

fn load_env() {
    dotenv().ok();
    let project_id = env::var("FIREBASE_PROJECT_ID").expect("FIREBASE_PROJECT_ID not set");
    println!("Firebase Project ID: {}", project_id);
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    load_env();

    let config = Config::from_env();

    log::info!("Firebase Project ID: {}", config.firebase_project_id);
    log::info!("Firebase Client Email: {}", config.firebase_client_email);

    let app = Router::new().route("/health", get(routes::health::health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    log::info!("Running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
