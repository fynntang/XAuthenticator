use serde::{ser::SerializeStruct, Serializer};

#[derive(thiserror::Error)]
pub enum CommonError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Master key not initialized")]
    MasterKeyNotInitialized,
    #[error("Invalid master key")]
    InvalidMasterKey,
    #[error("App not initialized")]
    AppNotInitialized,
    #[error("App is locked")]
    AppIsLocked,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Biometric authentication failed")]
    BiometricAuthFailed,
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Token expired")]
    TokenExpired,
}

impl From<CommonError> for String {
    fn from(err: CommonError) -> String {
        err.to_string()
    }
}

impl std::fmt::Debug for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl CommonError {
    pub fn code(&self) -> &'static str {
        match self {
            CommonError::UnexpectedError(_) => "UnexpectedError",
            CommonError::MasterKeyNotInitialized => "MasterKeyNotInitialized",
            CommonError::InvalidMasterKey => "InvalidMasterKey",
            CommonError::AppNotInitialized => "AppNotInitialized",
            CommonError::AppIsLocked => "AppIsLocked",
            CommonError::InvalidPassword => "InvalidPassword",
            CommonError::BiometricAuthFailed => "BiometricAuthFailed",
            CommonError::RequestError(_) => "RequestError",
            CommonError::TokenExpired => "TokenExpired",
        }
    }
}

impl serde::Serialize for CommonError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CommonError", 2)?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("reason", &self.to_string())?;
        state.end()
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
