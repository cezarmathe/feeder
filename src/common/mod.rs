pub mod errors;
pub mod report;

use std::time::{SystemTime, UNIX_EPOCH};

use rocket_contrib::json::Json;

/// Type that makes it easier to represent a Json result
pub type JsonResult<T> = Result<Json<T>, Json<errors::Error>>;

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
    ($opt: expr, $scope: ident, $error: expr) => {
        match $opt {
            Some(_value) => {
                return std::result::Result::Ok(_value);
            }
            None => {
                let err: crate::common::errors::Error;
                err = crate::common::errors::Error::new(
                    std::string::String::from($scope),
                    Box::new($error),
                );
                return std::result::Result::Err(err);
            }
        };
    };
    ($opt: expr, $scope: ident, $error: ident) => {
        match $opt {
            Some(_value) => {
                return std::result::Result::Ok(_value);
            }
            None => {
                let err: crate::common::errors::Error;
                err = crate::common::errors::Error::new(
                    std::string::String::from($scope),
                    Box::new($error),
                );
                return std::result::Result::Err(err);
            }
        };
    };
    ($opt: expr, $scope: literal, $error: expr) => {
        match $opt {
            Some(_value) => {
                return std::result::Result::Ok(_value);
            }
            None => {
                let err: crate::common::errors::Error;
                err = crate::common::errors::Error::new(
                    std::string::String::from($scope),
                    Box::new($error),
                );
                return std::result::Result::Err(err);
            }
        };
    };
    ($opt: expr, $scope: literal, $error: ident) => {
        match $opt {
            Some(_value) => {
                return std::result::Result::Ok(_value);
            }
            None => {
                let err: crate::common::errors::Error;
                err = crate::common::errors::Error::new(
                    std::string::String::from($scope),
                    Box::new($error),
                );
                return std::result::Result::Err(err);
            }
        };
    };
}

/// Create a new error using a scope and a message.
#[macro_export]
macro_rules! create_error {
    ($scope: ident, $error: ident) => {
        crate::common::errors::Error::new(String::from($scope), Box::new($error))
    };
    ($scope: literal, $error: ident) => {
        crate::common::errors::Error::new(String::from($scope), Box::new($error))
    };
    ($scope: ident, $error: expr) => {
        crate::common::errors::Error::new(String::from($scope), Box::new($error))
    };
    ($scope: literal, $error: expr) => {
        crate::common::errors::Error::new(String::from($scope), Box::new($error))
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
