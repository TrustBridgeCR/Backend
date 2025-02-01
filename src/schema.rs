// @generated automatically by Diesel CLI.

diesel::table! {
    escrows (id) {
        id -> Int4,
        loan_amount -> Int8,
        loan_term -> Varchar,
        purpose_of_loan -> Text,
        monthly_income -> Int8,
        status -> Varchar,
        sender_address -> Varchar,
        recipient_address -> Varchar,
        locked_funds -> Int8,
    }
}
