use actix_web::HttpRequest;
use chrono::{DateTime, Local, Utc};

#[derive(Debug)]
pub struct RequestContext {
    pub start_at: DateTime<Utc>,
    pub request: HttpRequest,
}

impl RequestContext {
    pub fn new(req: HttpRequest) -> Self {
        RequestContext {
            start_at: DateTime::from(Local::now()),
            request: req,
        }
    }
}
