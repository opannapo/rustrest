use crate::debug_info;
use crate::http_handler::schema::request::RequestContext;
use actix_web::body::{BodySize, BoxBody, MessageBody};
use actix_web::cookie::time::convert::Millisecond;
use actix_web::http::StatusCode;
use actix_web::web::Bytes;
use actix_web::HttpResponse;
use chrono::{Duration, Utc};
use serde::Serialize;
use sqlx::encode::IsNull;
use sqlx::encode::IsNull::No;
use std::error::Error;
use std::pin::Pin;
use std::ptr::null;
use std::sync::atomic::AtomicI32;
use std::task::{Context, Poll};

#[derive(Serialize)]
struct BaseResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
    pub duration: DurationSerializable,
}

#[derive(Serialize)]
struct DurationSerializable {
    secs: i64,
    nanos: i32,
    millis: f64,
}

impl From<Duration> for DurationSerializable {
    fn from(d: Duration) -> Self {
        let nanos = d.num_nanoseconds().unwrap_or(0);
        DurationSerializable {
            secs: d.num_seconds(),
            nanos: nanos as i32,
            millis: nanos as f64 / 1_000_000.0,
        }
    }
}

pub fn ok<T: Serialize>(context: RequestContext, t: T) -> HttpResponse {
    let time_end_duration: Duration = Utc::now().signed_duration_since(context.start_at);
    let duration_serializable = DurationSerializable::from(time_end_duration);
    let res = BaseResponse {
        data: Option::from(t),
        duration: DurationSerializable {
            secs: duration_serializable.secs,
            nanos: duration_serializable.nanos,
            millis: duration_serializable.millis,
        },
        error: None,
    };

    // let duration_serializable_m = duration_serializable;
    debug_info!(
        "req {:?} || duration secs {:?} nanos {:?} millis {:?}",
        context.request.path(),
        duration_serializable.secs,
        duration_serializable.nanos,
        duration_serializable.millis,
    );

    HttpResponse::Ok().json(res)
}

pub fn err(context: RequestContext, error: Box<dyn Error>) -> HttpResponse {
    let time_end_duration: Duration = Utc::now().signed_duration_since(context.start_at);
    let duration_serializable = DurationSerializable::from(time_end_duration);
    let res: BaseResponse<String> = BaseResponse {
        data: None,
        error: Option::from(error.to_string()),
        duration: DurationSerializable {
            secs: duration_serializable.secs,
            nanos: duration_serializable.nanos,
            millis: duration_serializable.millis,
        },
    };

    debug_info!(
        "req {:?} || duration secs {:?} nanos {:?} millis {:?}",
        context.request.path(),
        duration_serializable.secs,
        duration_serializable.nanos,
        duration_serializable.millis,
    );

    HttpResponse::Ok().json(res)
}
