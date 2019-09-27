use crate::{
    common::{JsonResult, report::Report},
    db::{feed, model::Feed},
    json_result,
};

use std::{option::Option, result::Result, str::FromStr, vec::Vec};

use log::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

const SCOPE: &str = "router/feeds";

#[get("/feeds/<uuid>?<with_items>")]
pub fn get_feed(uuid: String, with_items: Option<String>) -> JsonResult<Feed> {
    match Uuid::from_str(uuid.as_str()) {
        Ok(_value) => json_result!(feed::get_feed(_value)),
        Err(e) => {
            warn!("could not decode uuid: {:?}", e);
            json_result!(Result::Err(create_error!(SCOPE, "uuid is not valid")))
        }
    }
}

#[get("/feeds?<with_items>")]
pub fn get_feeds(with_items: Option<String>) -> JsonResult<Vec<Feed>> {
    json_result!(feed::get_feeds())
}

#[get("/feeds/<uuid>/checksum")]
pub fn get_feed_checksum(uuid: String) -> JsonResult<String> {
    match Uuid::from_str(uuid.as_str()) {
        Ok(_value) => json_result!(feed::get_feed_checksum(_value)),
        Err(e) => {
            warn!("could not decode uuid: {:?}", e);
            json_result!(Result::Err(create_error!(SCOPE, "uuid is not valid")))
        }
    }
}

#[post("/feeds", format = "application/json", data = "<_feed>")]
pub fn create_feed(_feed: Json<Feed>) -> JsonResult<Feed> {
    if _feed.title.is_none() {
        json_result!(Result::Err(create_error!(SCOPE, "model does not have a title")))
    }
    if _feed.description.is_none() {
        json_result!(Result::Err(create_error!(SCOPE, "model does not have a description")))
    }
    if _feed.link.is_none() {
        json_result!(Result::Err(create_error!(SCOPE, "model does not have a link")))
    }
    json_result!(feed::create_new_feed(_feed.0))
}

#[put("/feeds/<uuid>", format = "application/json", data = "<feed>")]
pub fn update_feed(uuid: String, feed: Json<Feed>) -> JsonResult<Feed> {
    unimplemented!();
}

#[delete("/feeds/<uuid>")]
pub fn delete_feed(uuid: String) -> JsonResult<Report<Feed>> {
    unimplemented!();
}
