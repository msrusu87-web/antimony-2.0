use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use crate::models::ErrorResponse;
use crate::db;

pub async fn register_worker(
    pool: web::Data<SqlitePool>,
    req: web::Json<serde_json::Value>,
) -> HttpResponse {
    if let (Some(worker_id), Some(miner_address)) = (
        req.get("worker_id").and_then(|v| v.as_str()),
        req.get("miner_address").and_then(|v| v.as_str()),
    ) {
        match db::register_mining_worker(pool.get_ref(), worker_id, miner_address).await {
            Ok(worker) => HttpResponse::Ok().json(worker),
            Err(e) => {
                log::error!("Register worker error: {}", e);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "REGISTRATION_FAILED".to_string(),
                    message: format!("Failed to register worker: {}", e),
                })
            }
        }
    } else {
        HttpResponse::BadRequest().json(ErrorResponse {
            error: "MISSING_FIELDS".to_string(),
            message: "worker_id and miner_address required".to_string(),
        })
    }
}

pub async fn get_workers(
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    match db::get_mining_workers(pool.get_ref()).await {
        Ok(workers) => HttpResponse::Ok().json(workers),
        Err(e) => {
            log::error!("Get workers error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: format!("Failed to get workers: {}", e),
            })
        }
    }
}

pub async fn get_pool_stats(
    pool: web::Data<SqlitePool>,
) -> HttpResponse {
    match db::get_pool_statistics(pool.get_ref()).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => {
            log::error!("Get pool stats error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: format!("Failed to get pool stats: {}", e),
            })
        }
    }
}

pub async fn get_payouts(
    pool: web::Data<SqlitePool>,
    address: web::Path<String>,
) -> HttpResponse {
    match sqlx::query_as::<_, crate::models::MiningPayout>(
        "SELECT id, miner_address, amount, created_at, confirmed FROM mining_payouts WHERE miner_address = ? ORDER BY created_at DESC"
    )
    .bind(address.as_str())
    .fetch_all(pool.get_ref())
    .await {
        Ok(payouts) => HttpResponse::Ok().json(payouts),
        Err(e) => {
            log::error!("Get payouts error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: format!("Failed to get payouts: {}", e),
            })
        }
    }
}
