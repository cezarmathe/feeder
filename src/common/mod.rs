pub mod error;
pub mod report;

use std::time::{SystemTime, UNIX_EPOCH};

use rocket_contrib::json::Json;

/// Type that makes it easier to represent a Json result
pub type JsonResult<T> = Result<Json<T>, Json<error::Error>>;

/// Match a Result<T, E> and return a Result<Json<T>, Json<E>>.
#[macro_export]
macro_rules! json_result {
    ($x: expr) => {
        match $x {
            Ok(_value) => std::result::Result::Ok(rocket_contrib::json::Json(_value)),
            Err(e) => std::result::Result::Err(rocket_contrib::json::Json(e)),
        }
    };
}

/// Create a new error using a scope and a message.
#[macro_export]
macro_rules! create_error {
    ($scope: ident, $message: literal) => {
        crate::common::error::Error::new(
            std::string::String::from($scope),
            std::string::String::from($message),
        )
    };
    ($scope: ident, $message: expr) => {
        crate::common::error::Error::new(
            std::string::String::from($scope),
            std::string::String::from($message),
        )
    };
    ($scope: literal, $message: literal) => {
        crate::common::error::Error::new(
            std::string::String::from($scope),
            std::string::String::from($message),
        )
    };
    ($scope: literal, $message: expr) => {
        crate::common::error::Error::new(
            std::string::String::from($scope),
            std::string::String::from($message),
        )
    };
}

/// Get the current timestamp(for reports and errors)
fn timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(_value) => _value.as_secs(),
        Err(e) => {
            // extremely bad if happens
            panic!(e);
        }
    }
}
