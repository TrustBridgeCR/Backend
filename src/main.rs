use crate::config::Config;
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};

mod config;
mod routes;
mod services;
mod models;
mod schema;
mod tests;
mod middleware;
mod utils;

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

   let app = Router::new()
       .route("/health", get(routes::health::health_check))
       .route("/login", post(routes::auth::login))
       .route(
        "/lender",
        get(routes::lender::handler)
            .layer(from_fn(middleware::auth::auth_middleware))
            .layer(from_fn(|req, next| middleware::auth::require_role(req, next, models::auth::UserRole::Lender)))
    )
       .route(
           "/borrower", 
           get(routes::borrower::handler)
               .layer(from_fn(middleware::auth::auth_middleware))
               .layer(from_fn(|req, next| middleware::auth::require_role(req, next, models::auth::UserRole::Borrower)))
       );

   let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
   log::info!("Running on http://{}", addr);

   axum::Server::bind(&addr)
       .serve(app.into_make_service())
       .await
       .unwrap();
}
    pretty_env_logger::init();

    load_env();

    let config = Config::from_env();

    // Log the Firebase and wallet configuration for debugging purposes
    log::info!("Firebase Project ID: {}", config.firebase_project_id);
    log::info!("Firebase Client Email: {}", config.firebase_client_email);
    log::info!("Albedo API Key: {}", config.albedo_api_key);
    log::info!("Freighter API Key: {}", config.freighter_api_key);

    // Initialize routes
    let app = Router::new()
        .route("/health", get(routes::health::health_check))
        .route("/wallet/authenticate", get(routes::wallet::authenticate_wallet))
        .route("/wallet/transfer", get(routes::wallet::transfer_funds));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    log::info!("Running on http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}