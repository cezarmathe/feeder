#[get("/feeds/<feed_uuid>/items/<item_uuid>")]
pub fn get_feed_item(feed_uuid: String, item_uuid: String) {
    unimplemented!();
}

#[get("/feeds/<feed_uuid>/items")]
pub fn get_feed_items(feed_uuid: String) {
    unimplemented!();
}

#[post("/feeds/<feed_uuid>/items")]
pub fn create_feed_item(feed_uuid: String) {
    unimplemented!();
}

#[put("/feeds/<feed_uuid>/items/<item_uuid>")]
pub fn update_feed_item(feed_uuid: String, item_uuid: String) {
    unimplemented!();
}

#[delete("/feeds/<feed_uuid>/items/<item_uuid>")]
pub fn delete_feed_item(feed_uuid: String, item_uuid: String) {
    unimplemented!();
}
