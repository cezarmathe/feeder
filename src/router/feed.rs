use crate::db::model;
use rocket_contrib::json::Json;

#[get("/feeds/<uuid>")]
pub fn get_feed(uuid: String) -> Json(model::Feed) {
    unimplemented!();
}

#[get("/feeds")]
pub fn get_feeds() {
    unimplemented!();
}

#[get("/feeds/<uuid>/checksum")]
pub fn get_feed_checksum(uuid: String) {
    unimplemented!();
}

#[post("/feeds", data=<feed>)]
pub fn create_feed(feed: model::Feed) {
    unimplemented!();
}

#[put("/feeds/<uuid>", data=<feed>)]
pub fn update_feed(uuid: String, feed: model::Feed) {
    unimplemented!();
}

#[delete("/feeds/<uuid>")]
pub fn delete_feed(uuid: String) {
    unimplemented!();
}
