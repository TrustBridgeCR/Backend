use crate::models::auth::{JWTClaims, UserRole};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono;
use crate::Config;

pub fn generate_token(user_id: String, role: UserRole) -> Result<String, jsonwebtoken::errors::Error> {
    let config = Config::from_env();
    let secret = config.jwt_secret.as_bytes();
    
    let claims = JWTClaims {
        sub: user_id,
        role,
        exp: chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .unwrap()
            .timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}