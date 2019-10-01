use crate::{common::error::Error, create_error, db::FeederDbConn};

use std::{option::Option, vec::Vec};

use crypto::{digest::Digest, sha3::Sha3};
use log::{debug, error, warn};
use mongodb::{coll::options::IndexModel, oid::ObjectId};
use uuid::Uuid;

const SCOPE: &str = "database/model";

/// Enum that specifies whether a feed contains only
/// the Uuids or the full items
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ItemsVec {
    Uuid(Vec<Uuid>),
    Full(Vec<FeedItem>),
}

#[derive(Clone, Debug, Deserialize, Model, Serialize)]
pub struct Feed {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", skip)]
    id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<Uuid>,

    pub title: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<aux::FeedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ItemsVec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    checksum: Option<String>,
}

impl Feed {
    /// Create a new feed.
    pub fn new(_title: &str, _description: &str, _link: &str) -> Result<Self, Error> {
        debug!(
            "creating a new feed struct with args: {:?}, {:?}, {:?}",
            _title, _description, _link
        );

        let title = String::from(_title);
        let description = String::from(_description);
        let link = String::from(_link);

        let mut feed = Feed {
            id: Option::None,
            uuid: Option::Some(Uuid::new_v4()),
            title: Option::Some(title),
            description: Option::Some(description),
            link: Option::Some(link),
            category: Option::None,
            copyright: Option::None,
            image: Option::None,
            language: Option::None,
            items: Option::None,
            checksum: Option::None,
        };

        // compute the checksum
        if !feed.compute_checksum(Option::None) {
            error!("checksum could not computed");
            return Result::Err(create_error!(
                SCOPE,
                "failed to compute the SHA256 checksum for the feed"
            ));
        }

        debug!("successfully created feed: {:?}", feed);
        return Result::Ok(feed);
    }

    /// Get the UUID of this feed
    pub fn get_uuid(&self) -> Option<Uuid> {
        return self.uuid.clone();
    }

    /// Get the checksum of this feed
    pub fn get_checksum(&self) -> Option<String> {
        return self.checksum.clone();
    }

    /// Compute the checksum of this feed
    /// The checksum is saved inside the object
    fn compute_checksum(&mut self, db_conn: Option<FeederDbConn>) -> bool {
        debug!("computing checksum for feed {:?}", self);

        let feed: Feed;

        if db_conn.is_some() {
            feed = self.clone().with_items(db_conn.unwrap());
        } else {
            feed = self.clone();
        }

        let json_result = serde_json::to_string(&feed);

        match json_result {
            Ok(json) => {
                debug!("successfully converted feed to json: {}", json);
                let mut hasher = Sha3::sha3_256();
                hasher.input_str(json.as_str());
                self.checksum = Some(String::from(hasher.result_str()));
                debug!("successfully computed the checksum");
                return true;
            }
            Err(e) => {
                warn!("could not convert feed {:?} to json: {:?}", feed, e);
                return false;
            }
        }
    }

    /// Return this feed along with its items
    fn with_items(mut self, db_conn: FeederDbConn) -> Self {
        if self.items.is_none() {
            return self;
        }

        let items_vec: ItemsVec = self.clone().items.unwrap();
        match items_vec {
            ItemsVec::Full(_) => self,
            ItemsVec::Uuid(items) => {
                let mut items_full: Vec<FeedItem> = Vec::new();

                //                for item in items {
                //
                //                }

                self
            }
        }
    }

    /// Return this feed with the items having only uuids
    fn with_uuids(mut self) -> Self {
        // TODO add some kind of error returning

        // If there are no items to be converted, return
        if self.items.is_none() {
            return self;
        }

        let items_vec: ItemsVec = self.clone().items.unwrap();
        match items_vec {
            ItemsVec::Uuid(_) => self,
            ItemsVec::Full(items) => {
                let mut items_uuid: Vec<Uuid> = Vec::new();

                for item in items {
                    if item.get_uuid().is_none() {
                        continue;
                    }
                    items_uuid.push(item.get_uuid().unwrap());
                }

                self.items = Option::Some(ItemsVec::Uuid(items_uuid));
                self
            }
        }
    }

    /// Generate the RSS representation of this feed.
    pub fn generate_rss(&self) {
        unimplemented!();
    }
}

#[derive(Clone, Debug, Deserialize, Model, Serialize)]
pub struct FeedItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<mongodb::oid::ObjectId>,
    uuid: Option<Uuid>,
    pub title: String,
    pub link: String,
    pub description: String,

    pub author: Option<String>,
    pub comments: Option<String>,
    pub enclosure: Option<aux::FeedItemEnclosure>,

    checksum: Option<String>,
}

impl FeedItem {
    pub fn get_uuid(&self) -> Option<Uuid> {
        self.uuid
    }
}

/// Module that contains auxiliary models
pub mod aux {
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FeedItemEnclosure {
        url: String,
        length: String,
        _type: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FeedImage {
        url: String,
        title: String,
        link: String,
    }
}

#[cfg(test)]
mod test {
    use super::Feed;

    #[test]
    fn feed_new_test() {
        *crate::_LOG;

        let _feed = Feed::new(
            "My title",
            "My example description for my feed test",
            "https://example.com",
        );
        if _feed.is_err() {
            panic!("Failed to create feed");
        }

        let feed = _feed.unwrap();

        println!("uuid: {}", feed.uuid.unwrap());
        println!("checksum: {}", feed.checksum.unwrap());
    }
}
