// Error types for Rosetta API
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::types::Error as RosettaError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Network not found: {0}")]
    NetworkNotFound(String),
    
    #[error("Block not found: {0}")]
    BlockNotFound(String),
    
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
    
    #[error("Invalid block identifier")]
    InvalidBlockIdentifier,
    
    #[error("Invalid transaction")]
    InvalidTransaction,
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

impl ApiError {
    pub fn to_rosetta_error(&self) -> RosettaError {
        match self {
            ApiError::NetworkNotFound(msg) => RosettaError {
                code: 1,
                message: format!("Network not found: {}", msg),
                retriable: false,
                details: None,
            },
            ApiError::BlockNotFound(msg) => RosettaError {
                code: 2,
                message: format!("Block not found: {}", msg),
                retriable: true,
                details: None,
            },
            ApiError::TransactionNotFound(msg) => RosettaError {
                code: 3,
                message: format!("Transaction not found: {}", msg),
                retriable: true,
                details: None,
            },
            ApiError::InvalidBlockIdentifier => RosettaError {
                code: 4,
                message: "Invalid block identifier".to_string(),
                retriable: false,
                details: None,
            },
            ApiError::InvalidTransaction => RosettaError {
                code: 5,
                message: "Invalid transaction".to_string(),
                retriable: false,
                details: None,
            },
            ApiError::Internal(msg) => RosettaError {
                code: 500,
                message: format!("Internal server error: {}", msg),
                retriable: true,
                details: None,
            },
            ApiError::NotImplemented(msg) => RosettaError {
                code: 501,
                message: format!("Not implemented: {}", msg),
                retriable: false,
                details: None,
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::NetworkNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BlockNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::TransactionNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::InvalidBlockIdentifier => StatusCode::BAD_REQUEST,
            ApiError::InvalidTransaction => StatusCode::BAD_REQUEST,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,
        };

        let rosetta_error = self.to_rosetta_error();
        (status, Json(rosetta_error)).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
