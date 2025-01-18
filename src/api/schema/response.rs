use crate::api::schema::request::RequestContext;
use crate::debug_info;
use actix_web::cookie::time::convert::Millisecond;
use actix_web::HttpResponse;
use chrono::{Duration, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct BaseResponse<T> {
    pub data: T,
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
        data: t,
        duration: DurationSerializable {
            secs: duration_serializable.secs,
            nanos: duration_serializable.nanos,
            millis: duration_serializable.millis,
        },
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
