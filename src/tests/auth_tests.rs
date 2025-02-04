#[cfg(test)]
mod tests {
   use axum::{
       body::Body,
       http::{Request, StatusCode},
       Router,
       routing::get,
       middleware::from_fn,
   };
   use tower::ServiceExt;
 use crate::env;
 use crate::Config;
   use crate::{utils::auth::generate_token, models::auth::UserRole, routes, middleware};

   fn test_app() -> Router {
       Router::new()
           .route("/lender", get(routes::lender::handler)
               .layer(from_fn(|req, next| middleware::auth::require_role(req, next, UserRole::Lender))))
           .route("/protected", get(routes::lender::handler)
               .layer(from_fn(|req, next| middleware::auth::require_role(req, next, UserRole::Admin))))
           .layer(from_fn(middleware::auth::auth_middleware))
   }

   fn setup() {
    env::set_var("JWT_SECRET", "test_secret");
    env::set_var("FIREBASE_PROJECT_ID", "test");
    env::set_var("FIREBASE_PRIVATE_KEY", "test");
    env::set_var("FIREBASE_CLIENT_EMAIL", "test");
    env::set_var("API_SECRET_KEY", "test");
}

#[tokio::test]
async fn test_auth_flow() {
    setup();
    let token = generate_token("test_user".to_string(), UserRole::Lender).unwrap();
    println!("Generated token: {}", token);
    
    let config = Config::from_env();
    println!("JWT Secret: {}", config.jwt_secret);

    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/lender")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

   #[tokio::test]
   async fn test_invalid_role() {
       let token = generate_token("test_user".to_string(), UserRole::Borrower).unwrap();
       let response = test_app()
           .oneshot(
               Request::builder()
                   .uri("/lender")
                   .header("Authorization", format!("Bearer {}", token))
                   .body(Body::empty())
                   .unwrap(),
           )
           .await
           .unwrap();
       assert_eq!(response.status(), StatusCode::FORBIDDEN);
   }

   #[tokio::test]
   async fn test_admin_access() {
       let token = generate_token("test_user".to_string(), UserRole::Admin).unwrap();
       let response = test_app()
           .oneshot(
               Request::builder()
                   .uri("/protected")
                   .header("Authorization", format!("Bearer {}", token))
                   .body(Body::empty())
                   .unwrap(),
           )
           .await
           .unwrap();
       assert_eq!(response.status(), StatusCode::OK);
   }
}