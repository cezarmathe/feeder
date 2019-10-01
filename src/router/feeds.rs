use crate::{
    common::{report::Report, JsonResult},
    db::{feed, model::Feed, FeederDbConn},
    json_result,
};

use std::{option::Option, result::Result, str::FromStr, vec::Vec};

use log::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

const SCOPE: &str = "router/feeds";

#[get("/feeds/<uuid>?<_with_items>")]
pub fn get_feed(
    db_conn: FeederDbConn,
    uuid: String,
    _with_items: Option<String>,
) -> JsonResult<Feed> {
    match Uuid::from_str(uuid.as_str()) {
        Ok(_value) => json_result!(feed::get_feed(db_conn, _value)), // TODO 29/09: check with_items
        Err(e) => {
            warn!("could not decode uuid: {:?}", e);
            json_result!(Result::Err(create_error!(SCOPE, "uuid is not valid")))
        }
    }
}

#[get("/feeds?<_with_items>")]
pub fn get_feeds(db_conn: FeederDbConn, _with_items: Option<String>) -> JsonResult<Vec<Feed>> {
    json_result!(feed::get_feeds(db_conn)) // TODO 29/09: check with_items
}

#[get("/feeds/<uuid>/checksum")]
pub fn get_feed_checksum(db_conn: FeederDbConn, uuid: String) -> JsonResult<String> {
    match Uuid::from_str(uuid.as_str()) {
        Ok(_value) => json_result!(feed::get_feed_checksum(db_conn, _value)),
        Err(e) => {
            warn!("could not decode uuid: {:?}", e);
            json_result!(Result::Err(create_error!(SCOPE, "uuid is not valid")))
        }
    }
}

#[post("/feeds", format = "application/json", data = "<model>")]
pub fn create_feed(db_conn: FeederDbConn, model: Json<Feed>) -> JsonResult<Feed> {
    if model.title.is_none() {
        json_result!(Result::Err(create_error!(
            SCOPE,
            "model does not have a title"
        )))
    }
    if model.description.is_none() {
        json_result!(Result::Err(create_error!(
            SCOPE,
            "model does not have a description"
        )))
    }
    if model.link.is_none() {
        json_result!(Result::Err(create_error!(
            SCOPE,
            "model does not have a link"
        )))
    }
    json_result!(feed::create_new_feed(db_conn, model.0))
}

#[put("/feeds/<_uuid>", format = "application/json", data = "<_feed>")]
pub fn update_feed(_uuid: String, _feed: Json<Feed>) -> JsonResult<Feed> {
    unimplemented!();
}

#[delete("/feeds/<_uuid>")]
pub fn delete_feed(_uuid: String) -> JsonResult<Report<Feed>> {
    unimplemented!();
}
