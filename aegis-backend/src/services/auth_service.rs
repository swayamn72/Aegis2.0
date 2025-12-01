use crate::utils::errors::AppError;
use crate::utils::validation::validate_password;
use anyhow::Result;
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub user_type: String,
    pub role: Option<String>,
    pub session_id: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
}

#[derive(Debug, Clone)]
pub enum UserType {
    Player,
    Admin,
    Organization,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TempTokenClaims {
    pub sub: String,
    pub user_type: String,
    pub token_type: String, // "reset_password" or "verify_email"
    pub exp: usize,
    pub iat: usize,
}

impl UserType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserType::Player => "player",
            UserType::Admin => "admin",
            UserType::Organization => "organization",
        }
    }
}

#[derive(Clone)]
pub struct AuthService {
    jwt_secret: String,
    jwt_expiration: i64,
    argon2: Argon2<'static>,
}

impl AuthService {
    pub fn new(jwt_secret: String, jwt_expiration: i64) -> Self {
        Self {
            jwt_secret,
            jwt_expiration,
            argon2: Argon2::default(),
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, AppError> {
        validate_password(password)?;

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::InternalServerError)?;

        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| AppError::InternalServerError)?;
        Ok(self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn generate_jwt(
        &self,
        user_id: Uuid,
        user_type: UserType,
        role: Option<String>,
        session_id: String, // ✅ FIXED: Use actual session ID
    ) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = (now + Duration::seconds(self.jwt_expiration)).timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            user_type: user_type.as_str().to_string(),
            role,
            session_id, // ✅ FIXED: Use actual session ID
            exp,
            iat: now.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::InternalServerError)
    }

    pub fn verify_jwt(&self, token: &str) -> Result<Claims, AppError> {
        println!("DEBUG: JWT verification - token length: {}", token.len());
        println!(
            "DEBUG: JWT verification - secret length: {}",
            self.jwt_secret.len()
        );

        let mut validation = Validation::default();
        validation.leeway = 0;

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        ) {
            Ok(data) => {
                println!("DEBUG: JWT verification successful");
                println!(
                    "DEBUG: Claims - sub: {}, exp: {}",
                    data.claims.sub, data.claims.exp
                );
                let now = chrono::Utc::now().timestamp() as usize;
                println!(
                    "DEBUG: Current time: {}, Token exp: {}",
                    now, data.claims.exp
                );
                Ok(data.claims)
            }
            Err(e) => {
                println!("DEBUG: JWT verification error: {:?}", e);
                Err(AppError::Unauthorized)
            }
        }
    }
    pub fn generate_temp_token(
        &self,
        user_id: Uuid,
        user_type: UserType,
        token_type: &str,
        expiry_hours: i64,
    ) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = (now + Duration::hours(expiry_hours)).timestamp() as usize;

        let claims = TempTokenClaims {
            sub: user_id.to_string(),
            user_type: user_type.as_str().to_string(),
            token_type: token_type.to_string(),
            exp,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::InternalServerError)
    }

    // Verify temporary token
    pub fn verify_temp_token(&self, token: &str) -> Result<TempTokenClaims, AppError> {
        let mut validation = Validation::default();
        validation.leeway = 0;

        match decode::<TempTokenClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        ) {
            Ok(data) => Ok(data.claims),
            Err(_) => Err(AppError::Validation("Invalid or expired token".to_string())),
        }
    }
}
