use xpassword_error::CommonError;

pub mod app_data_dir;

/// Validates password strength requirements
pub fn validate_password(password: &str) -> Result<(), CommonError> {
    if password.len() < 12 {
        return Err(CommonError::RequestError(
            "password must be at least 12 characters".to_string(),
        ));
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(CommonError::RequestError(
            "password must contain at least one uppercase letter".to_string(),
        ));
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(CommonError::RequestError(
            "password must contain at least one lowercase letter".to_string(),
        ));
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(CommonError::RequestError(
            "password must contain at least one digit".to_string(),
        ));
    }

    // Check for special characters (non-alphanumeric, excluding whitespace)
    if !password.chars().any(|c| !c.is_alphanumeric() && !c.is_whitespace()) {
        return Err(CommonError::RequestError(
            "password must contain at least one special character".to_string(),
        ));
    }

    Ok(())
}