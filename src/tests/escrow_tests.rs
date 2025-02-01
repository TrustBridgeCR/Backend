use crate::models::escrow::{self, Escrow, EscrowStatus};
use crate::services::escrow::EscrowService;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenvy::dotenv;

fn setup_test_db() -> EscrowService {
    dotenv().expect(".env file not found");

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    println!("Using database URL: {}", database_url); // Debug print
    EscrowService::new(&database_url)
}

#[tokio::test]
async fn test_create_escrow() {
    let service = setup_test_db();

    let test_escrow = Escrow {
        id: 0, // Will be set by database
        loan_amount: 1000,
        loan_term: "12 months".to_string(),
        purpose_of_loan: "Test loan".to_string(),
        monthly_income: 5000,
        status: "".to_string(),
        sender_address: "sender123".to_string(),
        recipient_address: "recipient456".to_string(),
        locked_funds: 0,
    };

    let result = service.create_escrow(test_escrow).await;
    assert!(result.is_ok());

    let created = result.unwrap();
    assert!(created.id > 0);
    assert_eq!(created.loan_amount, 1000);
    assert_eq!(created.status, EscrowStatus::Pending.to_string());
}

#[tokio::test]
async fn test_get_escrow() {
    let service = setup_test_db();

    let result = service.get_escrow(1).await;
    assert!(result.is_ok());
    let escrow = result.unwrap();
    assert!(escrow.id == 1);
}

#[tokio::test]
async fn test_lock_funds() {
    let service = setup_test_db();

    let result = service.lock_funds(1, 1000).await;
    assert!(result.is_ok());

    let escrow = result.unwrap();
    assert_eq!(escrow.locked_funds, 1000);
    assert_eq!(escrow.status, EscrowStatus::Funded.to_string());
}

#[tokio::test]
async fn test_release_funds() {
    let service = setup_test_db();

    let result = service.release_funds(1).await;
    assert!(result.is_ok());

    let escrow = result.unwrap();
    assert_eq!(escrow.status, EscrowStatus::Released.to_string());
}

#[tokio::test]
async fn test_cancel_and_refund() {
    let service = setup_test_db();

    let test_escrow = Escrow {
        id: 0,
        loan_amount: 1000,
        loan_term: "12 months".to_string(),
        purpose_of_loan: "Test loan".to_string(),
        monthly_income: 5000,
        status: "".to_string(),
        sender_address: "sender123".to_string(),
        recipient_address: "recipient456".to_string(),
        locked_funds: 0,
    };

    let created = service.create_escrow(test_escrow).await.unwrap();

    let result = service.cancel_and_refund(created.id).await;
    assert!(result.is_ok());

    let escrow = result.unwrap();
    assert_eq!(escrow.status, EscrowStatus::Cancelled.to_string());
    assert_eq!(escrow.locked_funds, 0);
}
