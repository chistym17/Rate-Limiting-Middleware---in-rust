use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use actix_service::{Service, Transform};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::middleware::RateLimiter;

pub struct RateLimitMiddleware<S>{
    service:Arc<Mutex<S>>,
}

