mod feed_items;
mod feeds;

use crate::{db::FeederDbConn, common::error::Error};

use rocket_contrib::json::Json;

const SCOPE: &str = "router";

#[catch(404)]
fn http_404() -> Json<Error> {
    let err: Error = create_error!(
        SCOPE,
        "404 not found"
    );
    Json(err)
}

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
        .register(catchers![http_404])
        .launch();
}
