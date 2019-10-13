mod catchers;
mod feed_items;
mod feeds;

use crate::{
    common::errors::{Error, UuidError},
    db::FeederDbConn,
};

use std::str::FromStr;

use uuid::Uuid;

const SCOPE: &str = "router";

/// Start the router
pub fn start() {
    rocket::ignite()
        .attach(FeederDbConn::fairing())
        .mount(
            "/",
            routes![
                feeds::get_feed,
                feeds::get_feeds,
                feeds::get_feed_checksum,
                feeds::create_feed,
                feeds::update_feed,
                feeds::delete_feed,
                feed_items::get_feed_item,
                feed_items::get_feed_items,
                feed_items::create_feed_item,
                feed_items::update_feed_item,
                feed_items::delete_feed_item,
            ],
        )
        .register(catchers![
            catchers::http_400_bad_request,
            catchers::http_401_unauthorized,
            catchers::http_403_forbidden,
            catchers::http_404_not_found,
            catchers::http_406_not_acceptable,
            catchers::http_500_internal_server_error,
            catchers::http_501_not_implemented,
            catchers::http_503_service_unavailable,
        ])
        .launch();
}

/// Check an Uuid
fn check_uuid(uuid: String, scope: &str) -> Result<Uuid, Error> {
    match Uuid::from_str(uuid.as_str()) {
        Ok(_value) => Result::Ok(_value),
        Err(e) => Result::Err(create_error!(
            scope,
            UuidError::UuidIsNotValid {
                err: format!("{:?}", e)
            }
        )),
    }
}
