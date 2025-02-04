use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub firebase_project_id: String,
    pub firebase_private_key: String,
    pub firebase_client_email: String,
    pub api_secret_key: String,
    pub stellar_network: String,
    pub stellar_horizon_url: String,
    pub stellar_escrow_public_key: String,
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
            api_secret_key: env::var("API_SECRET_KEY")
                .expect("Missing API_SECRET_KEY"),
            stellar_network: env::var("STELLAR_NETWORK")
                .unwrap_or_else(|_| "testnet".to_string()),
            stellar_horizon_url: env::var("STELLAR_HORIZON_URL")
                .unwrap_or_else(|_| "https://horizon-testnet.stellar.org".to_string()),
            stellar_escrow_public_key: env::var("STELLAR_ESCROW_PUBLIC_KEY")
                .expect("Missing STELLAR_ESCROW_PUBLIC_KEY"),
        }
    }
}