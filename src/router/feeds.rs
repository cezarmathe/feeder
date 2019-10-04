use crate::{
    common::{check_uuid, report::Report, JsonResult},
    db::{feed, model::Feed, FeederDbConn},
    json_result,
};

use std::{option::Option, result::Result, vec::Vec};

use rocket_contrib::json::Json;

const SCOPE: &str = "router/feeds";

#[get("/feeds/<uuid>?<_with_items>")]
pub fn get_feed(
    db_conn: FeederDbConn,
    uuid: String,
    _with_items: Option<String>,
) -> JsonResult<Feed> {
    match check_uuid(uuid, SCOPE) {
        Ok(_value) => json_result!(feed::get_feed(db_conn.clone(), _value)),
        Err(e) => json_result!(Result::Err(e)),
    }
}

#[get("/feeds?<_with_items>")]
pub fn get_feeds(db_conn: FeederDbConn, _with_items: Option<String>) -> JsonResult<Vec<Feed>> {
    json_result!(feed::get_feeds(db_conn.clone())) // TODO 29/09: check with_items
}

#[get("/feeds/<uuid>/checksum")]
pub fn get_feed_checksum(db_conn: FeederDbConn, uuid: String) -> JsonResult<String> {
    match check_uuid(uuid, SCOPE) {
        Ok(_value) => json_result!(feed::get_feed_checksum(db_conn.clone(), _value)),
        Err(e) => json_result!(Result::Err(e)),
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
    json_result!(feed::create_new_feed(db_conn.clone(), model.0))
}

#[put("/feeds/<uuid>", format = "application/json", data = "<feed>")]
pub fn update_feed(db_conn: FeederDbConn, uuid: String, feed: Json<Feed>) -> JsonResult<Feed> {
    match check_uuid(uuid, SCOPE) {
        Ok(_value) => json_result!(feed::update_feed(db_conn.clone(), _value, feed.0)),
        Err(e) => json_result!(Result::Err(e)),
    }
}

#[delete("/feeds/<uuid>")]
pub fn delete_feed(db_conn: FeederDbConn, uuid: String) -> JsonResult<Report<String>> {
    match check_uuid(uuid, SCOPE) {
        Ok(_value) => json_result!(feed::delete_feed(db_conn.clone(), _value)),
        Err(e) => json_result!(Result::Err(e)),
    }
}
