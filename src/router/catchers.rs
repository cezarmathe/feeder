use crate::common::error::Error;

use rocket::Request;
use rocket_contrib::json::Json;

fn catcher_json(scope: &str, message: &str) -> Json<Error> {
    let err: Error = create_error!(
        scope,
        message
    );
    Json(err)
}

#[catch(400)]
pub fn http_400_bad_request(request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, "bad request")
}

#[catch(401)]
pub fn http_401_unauthorized(request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, "unauthorized")
}

#[catch(403)]
pub fn http_403_forbidden(request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, "forbidden")
}

#[catch(404)]
pub fn http_404_not_found(request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, "not found")
}

#[catch(406)]
pub fn http_406_not_acceptable(request: &Request) -> Json<Error> {
    catcher_json(super::SCOPE, "not_acceptable")
}

#[catch(500)]
pub fn http_500_internal_server_error() -> Json<Error> {
    catcher_json(super::SCOPE, "internal server error")
}

#[catch(501)]
pub fn http_501_not_implemented() -> Json<Error> {
    catcher_json(super::SCOPE, "not implemented")
}

#[catch(503)]
pub fn http_503_service_unavailable() -> Json<Error> {
    catcher_json(super::SCOPE, "service unavailable")
}
