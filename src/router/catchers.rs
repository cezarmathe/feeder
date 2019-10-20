use crate::common::errors::{Error, HttpError};

use rocket::Request;
use rocket_contrib::json::Json;

fn catcher_json(scope: &str, message: HttpError) -> Json<Error> {
    let err: Error = create_error!(scope, message);
    Json(err)
}

#[catch(400)]
pub fn http_400_bad_request(_request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, HttpError::BadRequest)
}

#[catch(401)]
pub fn http_401_unauthorized(_request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, HttpError::Unauthorized)
}

#[catch(403)]
pub fn http_403_forbidden(_request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, HttpError::Forbidden)
}

#[catch(404)]
pub fn http_404_not_found(_request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, HttpError::NotFound)
}

#[catch(406)]
pub fn http_406_not_acceptable(_request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, HttpError::NotAcceptable)
}

#[catch(422)]
pub fn http_422_unprocessable_entity(_request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, HttpError::UnprocessableEntity)
}

#[catch(500)]
pub fn http_500_internal_server_error(_request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, HttpError::Ise)
}

#[catch(501)]
pub fn http_501_not_implemented(_request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, HttpError::NotImplemented)
}

#[catch(503)]
pub fn http_503_service_unavailable(_request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, HttpError::ServiceUnavailable)
}
