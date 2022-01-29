use serde::{Deserialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize,)]
pub struct BotId(pub String);
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize,)]
pub struct ServerId(pub String);
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize,)]
pub struct UserId(pub String);

/// Information about the server that attached to the bot.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BotServer {
    pub id: ServerId,
    pub approved: bool
}

/// Bot's stats.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BotStats {
    pub servers: u64,
    pub shards: u64,
    pub users: u64
}

/// Bot's stats.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BotLinks {
    pub discord: Option<String>,
    pub github: Option<String>,
    pub site: Option<String>
}

/// Information about the bot.
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

/// Model of a bot.
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