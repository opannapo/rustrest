use actix_web::http::StatusCode;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InternalError {
    pub http_status_code: StatusCode,
    pub message: String,
    pub code: String,
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.code, self.message)
    }
}

impl Error for InternalError {}

impl InternalError {
    pub fn db_exec(message: &str) -> Self {
        InternalError {
            code: "DB_EXEC".to_string(),
            http_status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.to_string(),
        }
    }
}
