use crate::{
    common::{check_uuid, report::Report, JsonResult},
    db::{feed, model::Feed, FeederDbConn},
    json_result,
};

use std::{option::Option, result::Result, vec::Vec};

use log::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

const SCOPE: &str = "router/feeds";

#[get("/feeds/<uuid>?<with_items>")]
pub fn get_feed(db_conn: FeederDbConn, uuid: String, with_items: Option<bool>) -> JsonResult<Feed> {
    // Check if the uuid is valid and return if it's not
    let good_uuid: Uuid;
    match check_uuid(uuid, SCOPE) {
        Ok(value) => good_uuid = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    // Get the feed from the database
    let mut feed: Feed;
    match feed::get_feed(db_conn.clone(), good_uuid) {
        Ok(value) => feed = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    // Check if the feed is requested with items or not
    if let None = with_items {
        json_result!(Result::Ok(feed))
    }
    if !with_items.unwrap() {
        json_result!(Result::Ok(feed))
    }

    debug!("feed requested with items");
    feed.with_items(db_conn.clone());
    json_result!(Result::Ok(feed))
}

#[get("/feeds?<with_items>")]
pub fn get_feeds(db_conn: FeederDbConn, with_items: Option<bool>) -> JsonResult<Vec<Feed>> {
    // Get the feed from the database
    let mut feeds: Vec<Feed>;
    match feed::get_feeds(db_conn.clone()) {
        Ok(value) => feeds = value,
        Err(e) => json_result!(Result::Err(e)),
    }

    // Check if the feed is requested with items or not
    if let None = with_items {
        json_result!(Result::Ok(feeds))
    }
    if !with_items.unwrap() {
        json_result!(Result::Ok(feeds))
    }

    debug!("feeds requested with items");
    // Get the feed items for each individual feed
    for feed in &mut feeds {
        feed.with_items(db_conn.clone());
    }

    json_result!(Result::Ok(feeds))
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
    // Filter out bad models
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

#[put("/feeds/<uuid>", format = "application/json", data = "<model>")]
pub fn update_feed(db_conn: FeederDbConn, uuid: String, model: Json<Feed>) -> JsonResult<Feed> {
    // Check if the uuid is valid and return if it's not
    let good_uuid: Uuid;
    match check_uuid(uuid, SCOPE) {
        Ok(value) => good_uuid = value,
        Err(e) => {
            json_result!(Result::Err(e));
        }
    }

    // Filter out bad models
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

    json_result!(feed::update_feed(db_conn.clone(), good_uuid, model.0))
}

#[delete("/feeds/<uuid>")]
pub fn delete_feed(db_conn: FeederDbConn, uuid: String) -> JsonResult<Report<String>> {
    match check_uuid(uuid, SCOPE) {
        Ok(value) => json_result!(feed::delete_feed(db_conn.clone(), value)),
        Err(e) => json_result!(Result::Err(e)),
    }
}
