use actix_http::{error::ErrorInternalServerError, Error};
use log::*;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Copy, Clone)]
pub struct LineMatch {
    pub left_from: u32,
    pub left_to: u32,
    pub right_from: u32,
    pub right_to: u32,
}

pub fn generate_uuid() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_simple()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_owned()
}

#[track_caller]
pub fn err<T: Display>(err: T) -> Error {
    let error_token = generate_uuid();
    let location = std::panic::Location::caller();
    error!("Error {} at {}: {}", error_token, location, err);
    ErrorInternalServerError(format!(
        "Please contact admin with error token {}",
        error_token
    ))
}