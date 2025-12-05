use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug)]
pub enum ApiError {
    WalletNotFound,
    TransactionNotFound,
    InsufficientBalance,
    InvalidAddress,
    InvalidPrivateKey,
    DatabaseError(String),
    InternalServerError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::WalletNotFound => StatusCode::NOT_FOUND,
            ApiError::TransactionNotFound => StatusCode::NOT_FOUND,
            ApiError::InsufficientBalance => StatusCode::BAD_REQUEST,
            ApiError::InvalidAddress => StatusCode::BAD_REQUEST,
            ApiError::InvalidPrivateKey => StatusCode::UNAUTHORIZED,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::WalletNotFound => {
                HttpResponse::NotFound().json(ApiErrorResponse {
                    error: "WALLET_NOT_FOUND".to_string(),
                    message: "Wallet not found".to_string(),
                })
            }
            ApiError::TransactionNotFound => {
                HttpResponse::NotFound().json(ApiErrorResponse {
                    error: "TRANSACTION_NOT_FOUND".to_string(),
                    message: "Transaction not found".to_string(),
                })
            }
            ApiError::InsufficientBalance => {
                HttpResponse::BadRequest().json(ApiErrorResponse {
                    error: "INSUFFICIENT_BALANCE".to_string(),
                    message: "Insufficient balance for transaction".to_string(),
                })
            }
            ApiError::InvalidAddress => {
                HttpResponse::BadRequest().json(ApiErrorResponse {
                    error: "INVALID_ADDRESS".to_string(),
                    message: "Invalid wallet address".to_string(),
                })
            }
            ApiError::InvalidPrivateKey => {
                HttpResponse::Unauthorized().json(ApiErrorResponse {
                    error: "INVALID_KEY".to_string(),
                    message: "Invalid private key".to_string(),
                })
            }
            ApiError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().json(ApiErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: msg.clone(),
                })
            }
            ApiError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(ApiErrorResponse {
                    error: "INTERNAL_ERROR".to_string(),
                    message: msg.clone(),
                })
            }
        }
    }
}
