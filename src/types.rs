use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize,)]
pub struct BotId(pub String);
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize,)]
pub struct ServerId(pub String);
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize,)]
pub struct UserId(pub String);

/// This model represents information about the server that attached to the bot.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BotServer {
    /// Id of server.
    pub id: ServerId,
    /// Is server approved?
    pub approved: bool
}

/// This model represents Bot's stats.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BotStats {
    /// Count of cached servers.
    pub servers: u64,
    /// Bot's shards count.
    pub shards: u64,
    /// Count of cached users.
    pub users: u64
}

/// This model represents Bot's social medias.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BotLinks {
    /// Bot's support server.
    pub discord: Option<String>,
    /// Bot's github repo.
    pub github: Option<String>,
    /// Bot's website.
    pub site: Option<String>
}

/// This model represents Information about the bot.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BotInformation {
    /// Bumps count.
    pub bumps: u64,
    /// How many times users have added the bot?
    pub added: u64,
    /// Bot's prefix.
    pub prefix: String,
    /// Bot's permissions.
    pub permissions: u64,
    /// Bot's search-tags.
    pub tags: Vec<String>,
    /// Bot's developers.
    pub developers: Vec<UserId>,
    /// Bot's social media.
    pub links: BotLinks,
    /// Bot's library/
    pub library: Option<String>,
    /// Bot's short description.
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    /// Bot's long description.
    #[serde(rename = "longDescription")]
    pub long_description: String,
    /// Bot's badge.
    pub badge: Option<u64>,
    /// Bot's stats.
    pub stats: BotStats,
    /// Bot's approval status.
    pub status: String
}

/// Model that represents a bot.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    /// Bot's Id.
    pub id: BotId,
    #[serde(rename = "shortCode")]
    /// Bot's page short code.
    pub short_code: Option<String>,
    pub links: Vec<String>,
    /// Bot's server.
    pub server: Option<BotServer>,
    /// Bot's information.
    pub information: BotInformation
}

/// This model represents single comment.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SingleComment {
    /// Comment author Id.
    #[serde(rename = "userID")]
    pub user_id: String,
    /// Comment content.
    pub text: String,
    /// Comment vote.
    pub vote: u64,
    /// Was comment updated?
    #[serde(rename = "isUpdated")]
    pub is_updated: bool,
    /// Comment creation timestamp.
    #[serde(rename = "created_at")]
    created_at: Option<u64>,
    /// Las edit timestamp.
    #[serde(rename = "updated_at")]
    updated_at: Option<u64>,
}