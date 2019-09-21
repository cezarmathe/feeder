use std::option::Option;
use std::vec::Vec;

pub struct Feed {
    title: String,
    description: OptionString,
    link: String,
    
    category: Option<String>,
    copyright: Option<String>,
    image: Option<FeedImage>,
    language: Option<String>,

    items: Vec<FeedItem>,
}

pub struct FeedImage {
    url: String,
    title: String,
    link: String
}

pub struct FeedItem {
    title: String,
    link: String,
    description: String,

    author: Option<String>,
    comments: Option<String>,
    enclosure: Option<FeedItemEnclosure>
}

pub struct FeedItemEnclosure {
    url: String,
    length: String,
    _type: String,
}