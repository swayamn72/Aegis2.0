use crate::utils::errors::AppError;

pub fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }
    if password.len() > 128 {
        return Err(AppError::Validation(
            "Password must be less than 128 characters".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(AppError::Validation(
            "Password must contain uppercase letter".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(AppError::Validation(
            "Password must contain lowercase letter".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(AppError::Validation(
            "Password must contain number".to_string(),
        ));
    }
    if !password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
    {
        return Err(AppError::Validation(
            "Password must contain special character".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), AppError> {
    let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !email_regex.is_match(email) {
        return Err(AppError::Validation("Invalid email format".to_string()));
    }
    Ok(())
}

pub fn validate_username(username: &str) -> Result<(), AppError> {
    if username.len() < 3 {
        return Err(AppError::Validation(
            "Username must be at least 3 characters".to_string(),
        ));
    }
    if username.len() > 30 {
        return Err(AppError::Validation(
            "Username must be less than 30 characters".to_string(),
        ));
    }
    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err(AppError::Validation(
            "Username can only contain letters, numbers, underscore, and dash".to_string(),
        ));
    }
    Ok(())
}
