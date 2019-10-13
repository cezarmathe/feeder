use crate::common::{
    errors::{Error, FeedItemDbError},
    report::Report,
};

use super::model::FeedItem;

use log::*;
use uuid::Uuid;
use wither::prelude::*;

const SCOPE: &str = "database/feed_item";
