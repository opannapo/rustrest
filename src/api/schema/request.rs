use actix_web::HttpRequest;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct RequestContext {
    pub start_at: DateTime<Utc>,
    pub request: HttpRequest,
}
