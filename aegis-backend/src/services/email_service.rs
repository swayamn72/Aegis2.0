// Update src/services/email_service.rs
use crate::config::settings::EmailConfig;
use crate::utils::errors::AppError;
use anyhow::Result;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[derive(Clone)]
pub struct EmailService {
    config: EmailConfig,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailService {
    pub fn new(config: EmailConfig) -> Result<Self, AppError> {
        // Skip SMTP setup if credentials are empty (development mode)
        if config.smtp_user.is_empty() || config.smtp_pass.is_empty() {
            tracing::warn!("SMTP credentials not configured, using development mode");
            return Ok(Self {
                config,
                mailer: AsyncSmtpTransport::<Tokio1Executor>::unencrypted_localhost(),
            });
        }

        let creds = Credentials::new(config.smtp_user.clone(), config.smtp_pass.clone());

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
            .map_err(|_| AppError::InternalServerError)?
            .port(config.smtp_port)
            .credentials(creds)
            .build();

        Ok(Self { config, mailer })
    }

    pub async fn send_password_reset(
        &self,
        to_email: &str,
        reset_token: &str,
    ) -> Result<(), AppError> {
        let reset_link = format!("http://localhost:5173/reset-password/{}", reset_token);

        // Development mode - just log
        if self.config.smtp_user.is_empty() {
            tracing::info!("Password reset link for {}: {}", to_email, reset_link);
            return Ok(());
        }

        // Production mode - send actual email
        let from: Mailbox = format!("{} <{}>", self.config.from_name, self.config.from_email)
            .parse()
            .map_err(|_| AppError::InternalServerError)?;

        let to: Mailbox = to_email
            .parse()
            .map_err(|_| AppError::InternalServerError)?;

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject("Password Reset Request - Aegis Gaming")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                r#"
                <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                    <div style="text-align: center; margin-bottom: 30px;">
                        <h1 style="color: #f59e0b; margin: 0;">Aegis Gaming</h1>
                    </div>
                    
                    <h2 style="color: #333; margin-bottom: 20px;">Password Reset Request</h2>
                    
                    <p style="color: #666; line-height: 1.6; margin-bottom: 20px;">
                        You requested a password reset for your Aegis Gaming account. Click the button below to set a new password:
                    </p>
                    
                    <div style="text-align: center; margin: 30px 0;">
                        <a href="{}" style="background-color: #f59e0b; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; font-weight: bold; display: inline-block;">
                            Reset Password
                        </a>
                    </div>
                    
                    <p style="color: #999; font-size: 14px; line-height: 1.6;">
                        This link will expire in 1 hour. If you didn't request this, you can safely ignore this email.
                    </p>
                    
                    <hr style="border: none; border-top: 1px solid #eee; margin: 30px 0;">
                    
                    <p style="color: #999; font-size: 12px; text-align: center;">
                        © 2024 Aegis Gaming. All rights reserved.
                    </p>
                </div>
                "#,
                reset_link
            ))
            .map_err(|_| AppError::InternalServerError)?;

        self.mailer.send(email).await.map_err(|e| {
            tracing::error!("Failed to send password reset email: {}", e);
            AppError::InternalServerError
        })?;

        tracing::info!("Password reset email sent to {}", to_email);
        Ok(())
    }

    pub async fn send_verification_email(
        &self,
        to_email: &str,
        verification_token: &str,
    ) -> Result<(), AppError> {
        let verification_link =
            format!("http://localhost:5173/verify-email/{}", verification_token);

        // Development mode - just log
        if self.config.smtp_user.is_empty() {
            tracing::info!(
                "Email verification link for {}: {}",
                to_email,
                verification_link
            );
            return Ok(());
        }

        // Production mode - send actual email
        let from: Mailbox = format!("{} <{}>", self.config.from_name, self.config.from_email)
            .parse()
            .map_err(|_| AppError::InternalServerError)?;

        let to: Mailbox = to_email
            .parse()
            .map_err(|_| AppError::InternalServerError)?;

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject("Verify Your Email - Aegis Gaming")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                r#"
                <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                    <div style="text-align: center; margin-bottom: 30px;">
                        <h1 style="color: #f59e0b; margin: 0;">Aegis Gaming</h1>
                    </div>
                    
                    <h2 style="color: #333; margin-bottom: 20px;">Welcome to Aegis Gaming!</h2>
                    
                    <p style="color: #666; line-height: 1.6; margin-bottom: 20px;">
                        Thank you for joining our gaming community. Please verify your email address by clicking the button below:
                    </p>
                    
                    <div style="text-align: center; margin: 30px 0;">
                        <a href="{}" style="background-color: #f59e0b; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; font-weight: bold; display: inline-block;">
                            Verify Email
                        </a>
                    </div>
                    
                    <p style="color: #999; font-size: 14px; line-height: 1.6;">
                        This link will expire in 24 hours. If you didn't create this account, you can safely ignore this email.
                    </p>
                    
                    <hr style="border: none; border-top: 1px solid #eee; margin: 30px 0;">
                    
                    <p style="color: #999; font-size: 12px; text-align: center;">
                        © 2024 Aegis Gaming. All rights reserved.
                    </p>
                </div>
                "#,
                verification_link
            ))
            .map_err(|_| AppError::InternalServerError)?;

        self.mailer.send(email).await.map_err(|e| {
            tracing::error!("Failed to send verification email: {}", e);
            AppError::InternalServerError
        })?;

        tracing::info!("Verification email sent to {}", to_email);
        Ok(())
    }
}
