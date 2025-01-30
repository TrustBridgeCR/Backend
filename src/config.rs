use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub firebase_project_id: String,
    pub firebase_private_key: String,
    pub firebase_client_email: String,
    pub api_secret_key: String,
    pub jwt_secret: String,
    pub albedo_api_key: String,
    pub freighter_api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            firebase_project_id: env::var("FIREBASE_PROJECT_ID")
                .expect("Missing FIREBASE_PROJECT_ID"),
            firebase_private_key: env::var("FIREBASE_PRIVATE_KEY")
                .expect("Missing FIREBASE_PRIVATE_KEY"),
            firebase_client_email: env::var("FIREBASE_CLIENT_EMAIL")
                .expect("Missing FIREBASE_CLIENT_EMAIL"),
            api_secret_key: env::var("API_SECRET_KEY").expect("Missing API_SECRET_KEY"),
            jwt_secret: env::var("JWT_SECRET").expect("Missing JWT_SECRET"),
            albedo_api_key: env::var("ALBEDO_API_KEY").expect("Missing ALBEDO_API_KEY"),
            freighter_api_key: env::var("FREIGHTER_API_KEY").expect("Missing FREIGHTER_API_KEY"),
        }
    }
}
