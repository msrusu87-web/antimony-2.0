use actix_web::{web, HttpResponse};
use sqlx::{SqlitePool, Row};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;
use totp_lite::{totp, Sha1};
use rand::Rng;
use crate::models::ErrorResponse;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<i64>,
    pub totp_secret: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
    pub requires_2fa: bool,
    pub totp_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFAVerifyRequest {
    pub email: String,
    pub password: String,
    pub totp_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enable2FARequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enable2FAResponse {
    pub success: bool,
    pub message: String,
    pub totp_secret: String,
    pub qr_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Confirm2FARequest {
    pub token: String,
    pub totp_code: String,
}

pub async fn register(
    pool: web::Data<SqlitePool>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    // Hash password
    let mut hasher = Sha256::new();
    hasher.update(req.password.as_bytes());
    let password_hash = hex::encode(hasher.finalize());

    // Generate TOTP secret but don't enable it yet
    let mut rng = rand::thread_rng();
    let secret_bytes: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
    let totp_secret = base32_encode(&secret_bytes);

    match sqlx::query(
        "INSERT INTO users (email, password_hash, totp_secret, is_admin, is_active, totp_enabled)
         VALUES (?, ?, ?, 0, 1, 0)"
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&totp_secret)
    .execute(pool.get_ref())
    .await {
        Ok(result) => {
            let user_id = result.last_insert_rowid();
            let token = generate_jwt_token(&req.email, false);

            HttpResponse::Ok().json(RegisterResponse {
                success: true,
                message: "Account created successfully! You can enable 2FA later for extra security.".to_string(),
                user_id: Some(user_id),
                totp_secret: Some(totp_secret),
                token: Some(token),
            })
        }
        Err(e) => {
            log::error!("Registration error: {}", e);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "REGISTRATION_FAILED".to_string(),
                message: if e.to_string().contains("UNIQUE constraint failed") {
                    "Email already registered".to_string()
                } else {
                    format!("Failed to register: {}", e)
                },
            })
        }
    }
}

pub async fn login(
    pool: web::Data<SqlitePool>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    // Hash password
    let mut hasher = Sha256::new();
    hasher.update(req.password.as_bytes());
    let password_hash = hex::encode(hasher.finalize());

    match sqlx::query(
        "SELECT password_hash, is_admin, totp_enabled FROM users WHERE email = ? AND is_active = 1"
    )
    .bind(&req.email)
    .fetch_optional(pool.get_ref())
    .await {
        Ok(Some(row)) => {
            let stored_hash: String = row.get("password_hash");
            let is_admin: bool = row.get("is_admin");
            let totp_enabled: bool = row.get("totp_enabled");

            if stored_hash == password_hash {
                if totp_enabled {
                    // 2FA is enabled, require code
                    HttpResponse::Ok().json(LoginResponse {
                        success: true,
                        message: "Password verified. Please provide 2FA code.".to_string(),
                        token: None,
                        requires_2fa: true,
                        totp_enabled: true,
                    })
                } else {
                    // No 2FA, login directly
                    let token = generate_jwt_token(&req.email, is_admin);
                    HttpResponse::Ok().json(LoginResponse {
                        success: true,
                        message: "Login successful".to_string(),
                        token: Some(token),
                        requires_2fa: false,
                        totp_enabled: false,
                    })
                }
            } else {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "INVALID_CREDENTIALS".to_string(),
                    message: "Invalid password".to_string(),
                })
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "USER_NOT_FOUND".to_string(),
            message: "Email not found or user inactive".to_string(),
        }),
        Err(e) => {
            log::error!("Login error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "LOGIN_FAILED".to_string(),
                message: "Database error".to_string(),
            })
        }
    }
}

pub async fn verify_2fa(
    pool: web::Data<SqlitePool>,
    req: web::Json<TwoFAVerifyRequest>,
) -> HttpResponse {
    // Hash password
    let mut hasher = Sha256::new();
    hasher.update(req.password.as_bytes());
    let password_hash = hex::encode(hasher.finalize());

    match sqlx::query(
        "SELECT password_hash, totp_secret, is_admin, totp_enabled FROM users WHERE email = ? AND is_active = 1"
    )
    .bind(&req.email)
    .fetch_optional(pool.get_ref())
    .await {
        Ok(Some(row)) => {
            let stored_hash: String = row.get("password_hash");
            let totp_secret: String = row.get("totp_secret");
            let is_admin: bool = row.get("is_admin");
            let totp_enabled: bool = row.get("totp_enabled");

            if stored_hash != password_hash {
                return HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "INVALID_CREDENTIALS".to_string(),
                    message: "Invalid password".to_string(),
                });
            }

            if !totp_enabled {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "2FA_NOT_ENABLED".to_string(),
                    message: "2FA is not enabled for this account".to_string(),
                });
            }

            // Verify TOTP
            if verify_totp(&totp_secret, &req.totp_code) {
                let token = generate_jwt_token(&req.email, is_admin);
                HttpResponse::Ok().json(LoginResponse {
                    success: true,
                    message: "Login successful".to_string(),
                    token: Some(token),
                    requires_2fa: false,
                    totp_enabled: true,
                })
            } else {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "INVALID_2FA".to_string(),
                    message: "Invalid 2FA code".to_string(),
                })
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "USER_NOT_FOUND".to_string(),
            message: "Email not found".to_string(),
        }),
        Err(e) => {
            log::error!("2FA verification error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "VERIFICATION_FAILED".to_string(),
                message: "Database error".to_string(),
            })
        }
    }
}

pub async fn enable_2fa(
    pool: web::Data<SqlitePool>,
    req: web::Json<Enable2FARequest>,
) -> HttpResponse {
    let email = extract_email_from_token(&req.token);
    
    match sqlx::query(
        "SELECT totp_secret, totp_enabled FROM users WHERE email = ?"
    )
    .bind(&email)
    .fetch_optional(pool.get_ref())
    .await {
        Ok(Some(row)) => {
            let totp_secret: String = row.get("totp_secret");
            let totp_enabled: bool = row.get("totp_enabled");

            if totp_enabled {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "2FA_ALREADY_ENABLED".to_string(),
                    message: "2FA is already enabled".to_string(),
                });
            }

            let qr_url = format!(
                "otpauth://totp/ATMN%20Wallet:{}?secret={}&issuer=ATMN%20Wallet",
                urlencoding::encode(&email),
                totp_secret
            );

            HttpResponse::Ok().json(Enable2FAResponse {
                success: true,
                message: "Scan QR code with Google Authenticator".to_string(),
                totp_secret,
                qr_url,
            })
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "USER_NOT_FOUND".to_string(),
            message: "User not found".to_string(),
        }),
        Err(e) => {
            log::error!("Enable 2FA error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "ENABLE_2FA_FAILED".to_string(),
                message: "Database error".to_string(),
            })
        }
    }
}

pub async fn confirm_2fa(
    pool: web::Data<SqlitePool>,
    req: web::Json<Confirm2FARequest>,
) -> HttpResponse {
    let email = extract_email_from_token(&req.token);
    
    match sqlx::query(
        "SELECT totp_secret FROM users WHERE email = ?"
    )
    .bind(&email)
    .fetch_optional(pool.get_ref())
    .await {
        Ok(Some(row)) => {
            let totp_secret: String = row.get("totp_secret");

            if verify_totp(&totp_secret, &req.totp_code) {
                // Enable 2FA
                match sqlx::query(
                    "UPDATE users SET totp_enabled = 1 WHERE email = ?"
                )
                .bind(&email)
                .execute(pool.get_ref())
                .await {
                    Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "message": "2FA enabled successfully"
                    })),
                    Err(e) => {
                        log::error!("Confirm 2FA update error: {}", e);
                        HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "CONFIRM_2FA_FAILED".to_string(),
                            message: "Failed to enable 2FA".to_string(),
                        })
                    }
                }
            } else {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "INVALID_2FA".to_string(),
                    message: "Invalid 2FA code".to_string(),
                })
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "USER_NOT_FOUND".to_string(),
            message: "User not found".to_string(),
        }),
        Err(e) => {
            log::error!("Confirm 2FA error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "CONFIRM_2FA_FAILED".to_string(),
                message: "Database error".to_string(),
            })
        }
    }
}

pub async fn disable_2fa(
    pool: web::Data<SqlitePool>,
    req: web::Json<Confirm2FARequest>,
) -> HttpResponse {
    let email = extract_email_from_token(&req.token);
    
    match sqlx::query(
        "SELECT totp_secret FROM users WHERE email = ?"
    )
    .bind(&email)
    .fetch_optional(pool.get_ref())
    .await {
        Ok(Some(row)) => {
            let totp_secret: String = row.get("totp_secret");

            if verify_totp(&totp_secret, &req.totp_code) {
                // Disable 2FA
                match sqlx::query(
                    "UPDATE users SET totp_enabled = 0 WHERE email = ?"
                )
                .bind(&email)
                .execute(pool.get_ref())
                .await {
                    Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "message": "2FA disabled successfully"
                    })),
                    Err(e) => {
                        log::error!("Disable 2FA update error: {}", e);
                        HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "DISABLE_2FA_FAILED".to_string(),
                            message: "Failed to disable 2FA".to_string(),
                        })
                    }
                }
            } else {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "INVALID_2FA".to_string(),
                    message: "Invalid 2FA code".to_string(),
                })
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "USER_NOT_FOUND".to_string(),
            message: "User not found".to_string(),
        }),
        Err(e) => {
            log::error!("Disable 2FA error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "DISABLE_2FA_FAILED".to_string(),
                message: "Database error".to_string(),
            })
        }
    }
}

// Helper functions
fn base32_encode(data: &[u8]) -> String {
    use base32::{Alphabet, encode};
    encode(Alphabet::RFC4648 { padding: false }, data)
}


fn verify_totp(secret: &str, code: &str) -> bool {
    match code.parse::<u32>() {
        Ok(code_num) => {
            let time_step = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() / 30;

            let expected = totp::<Sha1>(secret.as_bytes(), time_step);
            expected.to_string() == code_num.to_string()
        }
        Err(_) => false,
    }
}

fn generate_jwt_token(email: &str, is_admin: bool) -> String {
    format!("atmn_token_{}_admin_{}", email.replace("@", "_").replace(".", "_"), is_admin)
}

fn extract_email_from_token(token: &str) -> String {
    if let Some(email_part) = token.strip_prefix("atmn_token_") {
        if let Some(email_encoded) = email_part.split("_admin_").next() {
            let parts: Vec<&str> = email_encoded.split('_').collect();
            if parts.len() >= 2 {
                return format!("{}@{}", parts[0], parts[1..].join("."));
            }
        }
    }
    String::new()
}


pub async fn get_2fa_status(pool: web::Data<SqlitePool>, token: web::Path<String>) -> HttpResponse {
    // Extract email from token
    let email = extract_email_from_token(&token.into_inner());
    
    if email.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Invalid token format"
        }));
    }
    
    match sqlx::query("SELECT totp_enabled FROM users WHERE email = ?")
        .bind(&email)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(Some(row)) => {
            let totp_enabled: bool = row.try_get("totp_enabled").unwrap_or(false);
            HttpResponse::Ok().json(json!({
                "success": true,
                "totp_enabled": totp_enabled
            }))
        }
        Ok(None) => HttpResponse::NotFound().json(json!({
            "success": false,
            "message": "User not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Database error"
            }))
        }
    }
}
