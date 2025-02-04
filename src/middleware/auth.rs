use crate::models::auth::{JWTClaims, UserRole};
use axum::{http::{Request, StatusCode}, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::Config;

pub async fn auth_middleware<B>(
   mut req: Request<B>,
   next: Next<B>,
) -> Result<Response, (StatusCode, String)> {
   let config = Config::from_env();
   let auth_header = req.headers()
       .get(axum::http::header::AUTHORIZATION)
       .and_then(|h| h.to_str().ok())
       .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

   let token = auth_header
       .strip_prefix("Bearer ")
       .ok_or((StatusCode::UNAUTHORIZED, "Invalid Authorization header format".to_string()))?;

   let token_data = decode::<JWTClaims>(
       token,
       &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
       &Validation::default(),
   ).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

   req.extensions_mut().insert(token_data.claims);
   Ok(next.run(req).await)
}

pub async fn require_role<B>(
   req: Request<B>, 
   next: Next<B>,
   required_role: UserRole,
) -> Result<Response, StatusCode> {
   let claims = req.extensions().get::<JWTClaims>()
       .ok_or(StatusCode::UNAUTHORIZED)?;
       
   if claims.role != required_role {
       return Err(StatusCode::FORBIDDEN);
   }

   Ok(next.run(req).await)
}