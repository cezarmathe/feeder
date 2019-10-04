pub mod error;
pub mod report;

use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use rocket_contrib::json::Json;
use uuid::Uuid;

/// Type that makes it easier to represent a Json result
pub type JsonResult<T> = Result<Json<T>, Json<error::Error>>;

/// Match a Result<T, E> and return a Result<Json<T>, Json<E>>.
#[macro_export]
macro_rules! json_result {
    ($x: expr) => {
        match $x {
            Ok(value) => return std::result::Result::Ok(rocket_contrib::json::Json(value)),
            Err(e) => return std::result::Result::Err(rocket_contrib::json::Json(e)),
        };
    };
}

/// Unwrap an Option into a Result
#[macro_export]
macro_rules! option_to_result {
    ($opt: expr, $scope: ident, $message: ident) => {
        match $opt {
            Some(_value) => {
                return std::result::Result::Ok(_value);
            }
            None => {
                let err: crate::common::error::Error;
                err = create_error!($scope, $message);
                return std::result::Result::Err(err);
            }
        };
    };
    ($opt: expr, $scope: ident, $message: literal) => {
        match $opt {
            Some(_value) => {
                return std::result::Result::Ok(_value);
            }
            None => {
                let err: crate::common::error::Error;
                err = create_error!($scope, $message);
                return std::result::Result::Err(err);
            }
        };
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

pub fn check_uuid(uuid: String, scope: &str) -> Result<Uuid, error::Error> {
    match Uuid::from_str(uuid.as_str()) {
        Ok(_value) => Result::Ok(_value),
        Err(e) => {
            let err_msg = format!("uuid is not valid: {:?}", e);
            Result::Err(create_error!(scope, err_msg))
        }
    }
}
