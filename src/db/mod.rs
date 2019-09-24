pub mod feed;
pub mod model;

const DB_NAME: &str = "feeder";

#[macro_export]
macro_rules! get_db {
    () => {
        &crate::DB_CLIENT.db(crate::db::DB_NAME)
    };
}
