use super::check_uuid;

use crate::{
    common::{
        errors::{Error, FeedRouterError, ModelError},
        report::Report,
        JsonResult,
    },
    db::{model::Feed, DbConnection, FeedWrapper},
    json_result,
};

use log::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

const SCOPE: &str = "router/feeds";

/// Check a feed model used by create and update operations
fn check_feed_model(model: &Feed) -> Result<Json<()>, Json<Error>> {
    if model.title.is_none() {
        warn!("invalid model: no title");
        json_result!(Result::Err(create_error!(
            SCOPE,
            ModelError::ModelHasNoTitle
        )))
    }
    if model.description.is_none() {
        warn!("invalid model: no description");
        json_result!(Result::Err(create_error!(
            SCOPE,
            ModelError::ModelHasNoDescription
        )))
    }
    if model.link.is_none() {
        warn!("invalid model: no link");
        json_result!(Result::Err(create_error!(
            SCOPE,
            ModelError::ModelHasNoLink
        )))
    }
    info!("valid model");
    Result::Ok(Json(()))
}

#[get("/feeds/<uuid>?<_with_items>")]
pub fn get_feed(
    db_conn: DbConnection,
    uuid: String,
    _with_items: Option<bool>,
) -> JsonResult<Feed> {
    // Check if the uuid is valid and return if it's not
    let good_uuid: Uuid;
    match check_uuid(uuid, SCOPE) {
        Ok(value) => good_uuid = value,
        Err(e) => {
            warn!("invalid uuid received");
            json_result!(Result::Err(e));
        }
    }

    json_result!((&*db_conn).clone().get_feed(good_uuid))
}

#[get("/feeds/<uuid>/checksum")]
pub fn get_feed_checksum(db_conn: DbConnection, uuid: String) -> JsonResult<String> {
    match check_uuid(uuid, SCOPE) {
        Ok(value) => json_result!((&*db_conn).clone().get_feed_checksum(value)),
        Err(e) => {
            warn!("invalid uuid received");
            json_result!(Result::Err(e))
        }
    }
}

#[post("/feeds", format = "application/json", data = "<model>")]
pub fn create_feed(db_conn: DbConnection, model: Json<Feed>) -> JsonResult<Feed> {
    json_result!((&*db_conn).clone().create_feed(model.0))
}

#[put("/feeds?<uuid>", format = "application/json", data = "<model>")]
pub fn update_feed(
    db_conn: DbConnection,
    uuid: Option<String>,
    model: Json<Feed>,
) -> JsonResult<Feed> {
    // Check if the uuid is valid and return if it's not
    let mut good_uuid: Option<Uuid> = Option::None;

    if let Some(arg_uuid) = uuid {
        match check_uuid(arg_uuid, SCOPE) {
            Ok(value) => {
                info!("received uuid in the url arg");
                good_uuid = Option::Some(value)
            }
            Err(e) => {
                warn!("invalid uuid received");
                json_result!(Result::Err(e))
            }
        }
    }
    if let Some(model_uuid) = model.0.get_uuid() {
        match good_uuid {
            Some(arg_uuid) => {
                if arg_uuid != model_uuid {
                    warn!("url arg uuid does not match model uuid");
                    json_result!(Result::Err(create_error!(
                        SCOPE,
                        FeedRouterError::ModelAndArgUuidsDiffer
                    )))
                }
            }
            None => {
                info!("received uuid in the model");
                good_uuid = Option::Some(model_uuid)
            }
        }
    }

    if good_uuid.is_none() {
        warn!("no uuid received");
        json_result!(Result::Err(create_error!(SCOPE, FeedRouterError::NoUuid)))
    }

    check_feed_model(&model.0)?;

    json_result!((&*db_conn).clone().update_feed(good_uuid.unwrap(), model.0))
}

#[delete("/feeds/<uuid>")]
pub fn delete_feed(db_conn: DbConnection, uuid: String) -> JsonResult<Report<String>> {
    match check_uuid(uuid, SCOPE) {
        Ok(value) => json_result!((&*db_conn).clone().delete_feed(value)),
        Err(e) => {
            warn!("invalid uuid received");
            json_result!(Result::Err(e))
        }
    }
}
