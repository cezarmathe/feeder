mod catchers;
mod fairings;
mod feed_items;
mod feeds;

use crate::db::FeederDbConn;

const SCOPE: &str = "router";

/// Start the router
pub fn start() {
    rocket::ignite()
        .attach(FeederDbConn::fairing())
        .attach(fairings::RequestCounter::new())
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
