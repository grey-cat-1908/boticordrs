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

/// This model represents Server stats.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ServerStats {
    /// Server Id
    #[serde(rename = "serverID")]
    pub server_id: String,
    /// Is this bump request? (`1` - yes, `0` - no)
    pub up: u64,
    /// Is bot in server members list? (`1` - yes, `0` - no)
    pub status: u64,
    /// Server Name
    #[serde(rename = "serverName")]
    pub server_name: Option<String>,
    /// Server Icon
    #[serde(rename = "serverAvatar")]
    pub server_avatar: Option<String>,
    /// Server Members count (total, cached by the bot)
    #[serde(rename = "serverMembersAllCount")]
    pub server_members_all_count: Option<u64>,
    /// Online Server Members count (only currently online members count)
    #[serde(rename = "serverMembersOnlineCount")]
    pub server_members_online_count: Option<u64>,
    /// Server's Owner Id.
    #[serde(rename = "serverOwnerID")]
    pub server_owner_id: Option<String>,
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

/// This model represents Server's social medias.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ServerLinks {
    /// Server's invite.
    pub invite: Option<String>,
    /// Server's website.
    pub site: Option<String>,
    /// Server's youtube channel.
    pub youtube: Option<String>,
    /// Server's twitch channel.
    pub twitch: Option<String>,
    /// Server's steam profile.
    pub steam: Option<String>,
    /// Server's VK group.
    pub vk: Option<String>
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
    pub short_description: Option<String>,
    /// Bot's long description.
    #[serde(rename = "longDescription")]
    pub long_description: Option<String>,
    /// Bot's badge.
    pub badge: Option<String>,
    /// Bot's stats.
    pub stats: BotStats,
    /// Bot's approval status.
    pub status: String
}

/// This model represents Information about the server.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerInformation {
    /// Server's name.
    pub name: String,
    /// Server's avatar.
    pub avatar: Option<String>,
    /// Server's members count
    pub members: Option<Vec<u64>>,
    /// Server's owner.
    pub owner: Option<UserId>,
    /// Bumps count.
    pub bumps: u64,
    /// Server's search-tags.
    pub tags: Vec<String>,
    /// Server's social media.
    pub links: Option<ServerLinks>,
    /// Server's short description.
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
    /// Server's long description.
    #[serde(rename = "longDescription")]
    pub long_description: Option<String>,
    /// Server's badge.
    pub badge: Option<String>,
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
    pub links: Option<Vec<String>>,
    /// Bot's server.
    pub server: Option<BotServer>,
    /// Bot's information.
    pub information: BotInformation
}

/// Model that represents a server.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    /// Server's Id.
    pub id: ServerId,
    #[serde(rename = "shortCode")]
    /// Server's page short code.
    pub short_code: Option<String>,
    pub links: Option<Vec<String>>,
    /// Server's status.
    pub status: String,
    /// Information about the server.
    pub information: ServerInformation
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

/// This model represents single bot.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SingleUserBot {
    /// Bot's id.
    pub id: BotId,
    /// Bot's page shortcode.
    #[serde(rename = "shortCode")]
    pub short_code: Option<String>,
}

/// This model represents information about user.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct UserInformation {
    /// Id of user
    pub id: UserId,
    /// Custom status
    pub status: Option<String>,
    /// User badge
    pub badge: Option<String>,
    /// User Page shortcode
    #[serde(rename = "shortCode")]
    pub short_code: Option<String>,
    /// User's Website
    pub site: Option<String>,
    /// User's VK page
    pub vk: Option<String>,
    /// User's steam profile
    pub steam: Option<String>,
    /// User's youtube channel
    pub youtube: Option<String>,
    /// User's twitch account
    pub twitch: Option<String>,
    /// User's githup profile
    pub git: Option<String>
}

/// This model represents user's comments
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct UserComments {
    /// Comments on bots pages
    pub bots: Option<Vec<SingleComment>>,
    /// Comments on user pages
    pub servers: Option<Vec<SingleComment>>
}