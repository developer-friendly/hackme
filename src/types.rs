use std::path::PathBuf;

use serde::{Deserialize, Serialize};

type ItemId = u32;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    about: Option<String>,
    pub submitted: Option<Vec<ItemId>>,
    created: u32,
    pub id: String,
    karma: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ItemType {
    Job,
    Story,
    Comment,
    Poll,
    PollOpt,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Item {
    id: ItemId,
    #[serde(default)]
    deleted: bool,
    #[serde(alias = "type")]
    type_: Option<ItemType>,
    by: Option<String>,
    time: Option<u32>,
    text: Option<String>,
    #[serde(default)]
    dead: bool,
    parent: Option<ItemId>,
    poll: Option<ItemId>,
    kids: Option<Vec<ItemId>>,
    url: Option<String>,
    score: Option<i32>,
    title: Option<String>,
    parts: Option<Vec<ItemId>>,
    descendants: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DbData {
    pub user: User,
    pub items: Vec<Item>,
}
