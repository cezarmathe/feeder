use std::option::Option;
use std::str;
use std::vec::Vec;

use crypto::digest::Digest;
use crypto::sha3::Sha3;
use log::{debug, error, warn};
use mongodb::{
    coll::options::IndexModel,
    oid::ObjectId
};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Model, Serialize)]
pub struct Feed {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id: Option<ObjectId>,
    uuid: Uuid,
    title: String,
    description: String,
    link: String,

    #[serde(skip_serializing_if="Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub copyright: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub image: Option<FeedImage>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    items_uuid: Vec<Uuid>,

    #[serde(skip_serializing_if="Option::is_none")]
    checksum: Option<String>,
}

impl Feed {
    /// Create a new feed.
    pub fn new(_title: &str, _description: &str, _link: &str) -> Option<Self> {
        debug!(
            "creating a new feed struct with args: {:?}, {:?}, {:?}",
            _title, _description, _link
        );

        let title = String::from(_title);
        let description = String::from(_description);
        let link = String::from(_link);

        let mut feed = Feed {
            id: Option::None,
            uuid: Uuid::new_v4(),
            title,
            description,
            link,
            category: Option::None,
            copyright: Option::None,
            image: Option::None,
            language: Option::None,
            items_uuid: Vec::new(),
            checksum: Option::None,
        };

        // compute the checksum
        if !feed.compute_checksum() {
            error!("checksum not computed, returning Option::None");
            return Option::None;
        }

        debug!("successfully created feed: {:?}", feed);
        return Option::Some(feed);
    }

    pub fn to_json(&self) -> Option<String> {
        match serde_json::to_string(self) {
            Ok(_result) => {
                Option::Some(_result)
            },
            Err(e) => {
                warn!("failed to convert feed {:?} to json: {:?}", self, e);
                Option::None
            }
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid.clone();
    }

    pub fn get_checksum(&self) -> Option<String> {
        return self.checksum.clone();
    }

    /// Compute the checksum of this feed.
    /// The checksum is saved inside the object.
    fn compute_checksum(&mut self) -> bool {
        debug!("computing checksum for feed {:?}", self);
        let json_result = serde_json::to_string(self);

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
                warn!("could not convert feed {:?} to json: {:?}", self, e);
                return false;
            }
        }
    }

    /// Generate the RSS representation of this feed.
    pub fn generate_rss(&self) {
        unimplemented!();
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeedImage {
    url: String,
    title: String,
    link: String,
}

#[derive(Clone, Debug, Deserialize, Model, Serialize)]
pub struct FeedItem {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id: Option<mongodb::oid::ObjectId>,
    uuid: Option<Uuid>,
    title: String,
    link: String,
    description: String,

    author: Option<String>,
    comments: Option<String>,
    enclosure: Option<FeedItemEnclosure>,

    checksum: Option<String>,
}

impl FeedItem {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeedItemEnclosure {
    url: String,
    length: String,
    _type: String,
}

#[cfg(test)]
mod test {
    use super::Feed;

    #[test]
    fn feed_new_test() {
        *crate::LOG;

        let _feed = Feed::new(
            "My title",
            "My example description for my feed test",
            "https://example.com",
        );
        if _feed.is_none() {
            panic!("Failed to create feed");
        }

        let feed = _feed.unwrap();

        println!("uuid: {}", feed.uuid.unwrap());
        println!("checksum: {}", feed.checksum.unwrap());
    }
}
