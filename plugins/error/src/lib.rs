use serde::Serializer;

#[derive(thiserror::Error)]
pub enum CommonError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
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

impl serde::Serialize for CommonError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the error as a human-readable string, including its message.
        // This avoids requiring inner error types (e.g., sea_orm::DbErr, anyhow::Error)
        // to be serializable while still providing useful information to the frontend.
        serializer.serialize_str(&self.to_string())
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
