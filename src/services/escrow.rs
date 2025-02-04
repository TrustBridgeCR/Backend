use crate::models::escrow::{Escrow, EscrowStatus};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use stellar_sdk::{Client, Network, Keypair, TransactionBuilder};
use std::str::FromStr;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct StellarConfig {
    pub network: Network,
    pub horizon_url: String,
    pub escrow_keypair: Keypair,
}

impl StellarConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        
        let network = match std::env::var("STELLAR_NETWORK")
            .unwrap_or_else(|_| "testnet".to_string()).as_str() {
            "mainnet" => Network::Public,
            _ => Network::Testnet,
        };

        let horizon_url = std::env::var("STELLAR_HORIZON_URL")
            .unwrap_or_else(|_| "https://horizon-testnet.stellar.org".to_string());

        let secret_key = std::env::var("STELLAR_ESCROW_SECRET_KEY")
            .expect("STELLAR_ESCROW_SECRET_KEY must be set");

        Self {
            network,
            horizon_url,
            escrow_keypair: Keypair::from_secret_seed(&secret_key)
                .expect("Invalid Stellar secret key"),
        }
    }

    pub fn create_client(&self) -> Client {
        Client::new(&self.horizon_url, self.network)
    }
}

pub struct EscrowService {
    pool: DbPool,
    stellar_config: StellarConfig
}

impl EscrowService {
    pub fn new(database_url: &str, stellar_config: StellarConfig) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        
        EscrowService { 
            pool,
            stellar_config 
        }
    }

    pub async fn create_stellar_escrow(&self, new_escrow: Escrow) -> Result<Escrow, String> {
        let db_escrow = self.create_escrow(new_escrow).await?;
        let client = self.stellar_config.create_client();

        let source_account = client.load_account(&self.stellar_config.escrow_keypair.public_key()).map_err(|e| format!("Failed to load Stellar account: {:?}", e))?;

        let transaction = TransactionBuilder::new(&source_account, &self.stellar_config.network)
            .add_operation(Operation::Payment {
                destination: Keypair::from_public_key(&db_escrow.recipient_public_key).map_err(|e| format!("Invalid recipient key: {:?}", e))?,
                asset: stellar_sdk::Asset::native(),
                amount: db_escrow.loan_amount as f64,
            })
            .add_memo(Memo::Text("Escrow Transaction"))
            .build()
            .map_err(|e| format!("Failed to build Stellar transaction: {:?}", e))?;

        let signed_tx = transaction.sign(&self.stellar_config.escrow_keypair, &self.stellar_config.network);

        let response = client.submit_transaction(&signed_tx).map_err(|e| format!("Failed to submit Stellar transaction: {:?}", e))?;

        println!("Stellar transaction successful: {:?}", response);
        
        Ok(db_escrow)
    }

    pub async fn create_escrow(&self, new_escrow: Escrow) -> Result<Escrow, String> {
        use crate::schema::escrows::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        // Validate the escrow
        if new_escrow.loan_amount <= 0 {
            return Err("Loan amount must be greater than 0".to_string());
        }

        if new_escrow.loan_term.is_empty() {
            return Err("Loan term must be provided".to_string());
        }

        if new_escrow.purpose_of_loan.is_empty() {
            return Err("Purpose of loan must be provided".to_string());
        }

        if new_escrow.monthly_income <= 0 {
            return Err("Monthly income must be greater than 0".to_string());
        }

        // Set initial status to Pending if not set
        let mut escrow_to_create = new_escrow;
        if escrow_to_create.status.is_empty() {
            escrow_to_create.status = EscrowStatus::Pending.to_string();
        }

        // Create the escrow
        diesel::insert_into(escrows)
            .values(&escrow_to_create)
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to create escrow: {}", e))
    }

    pub async fn get_escrow(&self, _id: i32) -> Result<Escrow, String> {
        use crate::schema::escrows::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        escrows
            .find(_id)
            .first(&mut conn)
            .map_err(|_| "Escrow not found".to_string())
    }

    pub async fn update_status(&self, _id: i32, new_status: EscrowStatus) -> Result<Escrow, String> {
        use crate::schema::escrows::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        // First get the current escrow
        let current_escrow = escrows
            .find(_id)
            .first::<Escrow>(&mut conn)
            .map_err(|_| "Escrow not found".to_string())?;

        // Validate status transition
        match EscrowStatus::from_string(&current_escrow.status) {
            Ok(EscrowStatus::Released) | Ok(EscrowStatus::Cancelled) => {
                return Err("Cannot update status of completed escrow".to_string());
            }
            _ => {}
        }

        // Update the status
        diesel::update(escrows.find(_id))
            .set(status.eq(new_status.to_string()))
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to update status: {}", e))
    }

    pub async fn lock_funds(&self, _id: i32, amount: i64) -> Result<Escrow, String> {
        use crate::schema::escrows::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let escrow: Escrow = escrows
            .find(_id)
            .first(&mut conn)
            .map_err(|_| "Escrow not found".to_string())?;

        if escrow.status != EscrowStatus::Pending.to_string() {
            return Err("Escrow must be in PENDING status to lock funds".to_string());
        }

        diesel::update(escrows.find(_id))
            .set((
                locked_funds.eq(amount),
                status.eq(EscrowStatus::Funded.to_string()),
            ))
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to lock funds: {}", e))
    }

    pub async fn release_funds(&self, _id: i32) -> Result<Escrow, String> {
        use crate::schema::escrows::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let escrow: Escrow = escrows
            .find(_id)
            .first(&mut conn)
            .map_err(|_| "Escrow not found".to_string())?;

        if escrow.status != EscrowStatus::Funded.to_string() {
            return Err("Escrow must be in FUNDED status to release funds".to_string());
        }

        diesel::update(escrows.find(_id))
            .set(status.eq(EscrowStatus::Released.to_string()))
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to release funds: {}", e))
    }

    pub async fn cancel_and_refund(&self, _id: i32) -> Result<Escrow, String> {
        use crate::schema::escrows::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let escrow: Escrow = escrows
            .find(_id)
            .first(&mut conn)
            .map_err(|_| "Escrow not found".to_string())?;

        if escrow.status != EscrowStatus::Pending.to_string()
            && escrow.status != EscrowStatus::Funded.to_string()
        {
            return Err("Cannot cancel escrow in current status".to_string());
        }

        diesel::update(escrows.find(_id))
            .set((
                status.eq(EscrowStatus::Cancelled.to_string()),
                locked_funds.eq(0),
            ))
            .get_result(&mut conn)
            .map_err(|e| format!("Failed to cancel escrow: {}", e))
    }
}
