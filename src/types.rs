use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize,)]
pub struct BotId(pub String);
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize,)]
pub struct ServerId(pub String);
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize,)]
pub struct UserId(pub String);

/// This model represents a server (attached to the bot)
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BotServer {
    pub id: ServerId,
    pub approved: bool
}

/// This model represents bot's stats.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BotStats {
    pub servers: u64,
    pub shards: u64,
    pub users: u64
}

/// This model represents single comment.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SingleComment {
    #[serde(rename = "userID")]
    pub user_id: String,
    pub text: String,
    pub vote: u64,
    #[serde(rename = "isUpdated")]
    pub is_updated: bool,
    #[serde(rename = "created_at")]
    created_at: Option<u64>,
    #[serde(rename = "updated_at")]
    updated_at: Option<u64>,
}

/// This model represents bot's social media links.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BotLinks {
    pub discord: Option<String>,
    pub github: Option<String>,
    pub site: Option<String>
}


/// This model represents information about the bot.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BotInformation {
    pub bumps: u64,
    pub added: u64,
    pub prefix: String,
    pub permissions: u64,
    pub tags: Vec<String>,
    pub developers: Vec<UserId>,
    pub links: BotLinks,
    pub library: Option<String>,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "longDescription")]
    pub long_description: String,
    pub badge: Option<u64>,
    pub stats: BotStats,
    pub status: String
}

/// This model represents bot.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    pub id: BotId,
    #[serde(rename = "shortCode")]
    pub short_code: Option<String>,
    pub links: Vec<String>,
    pub server: BotServer,
    pub information: BotInformation
}
