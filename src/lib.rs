//! # boticordrs
//!
//! Crate for [Boticord](https://boticord.top/) API
//! ## Usage
//!
//! Add this to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! boticordrs = "0.1.3"
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use boticordrs::{BoticordClient};
//! use boticordrs::types::{BotStats};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = BoticordClient::new("your token".to_string(), 2).expect("failed client");
//!
//!     let stats = BotStats {servers: 2514, shards: 3, users: 338250};
//!
//!     match client.post_bot_stats(stats).await {
//!         Ok(_) => {
//!             println!("Well Done!")
//!         },
//!         Err(e) => eprintln!("{}", e),
//!     }
//! }
//! ```
#![doc(html_root_url = "https://docs.rs/boticordrs/0.1.3")]

use reqwest::header::AUTHORIZATION;
use reqwest::{Client as ReqwestClient, Response};
use reqwest::{Method};

macro_rules! api_url {
    ($e:expr) => {
        concat!("https://api.boticord.top/v", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(api_url!($e), $($rest)*)
    };
}

pub mod types;
mod errors;

use types::*;
pub use errors::BoticordError;

/// You can use it to make it much easier to use the Boticord API.
#[derive(Clone)]
pub struct BoticordClient {
    client: ReqwestClient,
    token: String,
    version: u64
}

impl BoticordClient {
    /// Constructs a new Client.
    ///
    /// # Arguments
    ///
    /// * `token` - Your BotiCord token
    /// * `version` - Version of BotiCord API
    ///
    pub fn new(token: String, version: u64) -> Result<Self, BoticordError> {
        let client = ReqwestClient::builder().build().map_err(errors::from)?;
        Ok(BoticordClient { client, token, version })
    }

    /// Constructs a new Client with ReqwestClient specified by user.
    ///
    /// # Arguments
    ///
    /// * `client` - Your custom ReqwestClient
    /// * `token` - Your BotiCord token
    /// * `version` - Version of BotiCord API
    ///
    pub fn new_with_client(client: ReqwestClient, token: String, version: u64) -> Self {
        BoticordClient { client, token, version }
    }

    /// Get information about a specific bot.
    ///
    /// # Arguments
    ///
    /// * `bot` - Id of bot.
    ///
    pub async fn get_bot_info(&self, bot: String) -> Result<Bot, BoticordError> {
        let url = api_url!("{}/bot/{}", &self.version, bot);
        get(self, url).await
    }

    /// Get information about a specific server.
    ///
    /// # Arguments
    ///
    /// * `server` - Id of server.
    ///
    pub async fn get_server_info(&self, server: String) -> Result<Server, BoticordError> {
        let url = api_url!("{}/server/{}", &self.version, server);
        get(self, url).await
    }

    /// Get information about a specific user.
    ///
    /// # Arguments
    ///
    /// * `user` - Id of user.
    ///
    pub async fn get_user_info(&self, user: String) -> Result<UserInformation, BoticordError> {
        let url = api_url!("{}/profile/{}", &self.version, user);
        get(self, url).await
    }

    /// Get Vec of bot's comments.
    ///
    /// # Arguments
    ///
    /// * `bot` - Id of bot.
    ///
    pub async fn get_bot_comments(&self, bot: String) -> Result<Vec<SingleComment>, BoticordError> {
        let url = api_url!("{}/bot/{}/comments", &self.version, bot);
        get(self, url).await
    }

    /// Get Vec of server's comments.
    ///
    /// # Arguments
    ///
    /// * `server` - Id of server.
    ///
    pub async fn get_server_comments(&self,
                                     server: String
    ) -> Result<Vec<SingleComment>, BoticordError> {
        let url = api_url!("{}/server/{}/comments", &self.version, server);
        get(self, url).await
    }

    /// Get Vec of user's comments.
    ///
    /// # Arguments
    ///
    /// * `user` - Id of user.
    ///
    pub async fn get_user_comments(&self,
                                   user: String
    ) -> Result<UserComments, BoticordError> {
        let url = api_url!("{}/profile/{}/comments", &self.version, user);
        get(self, url).await
    }

    /// Get Vec of user's bots.
    ///
    /// # Arguments
    ///
    /// * `user` - Id of user.
    ///
    pub async fn get_user_bots(&self,
                               user: String
    ) -> Result<Vec<SingleUserBot>, BoticordError> {
        let url = api_url!("{}/bots/{}", &self.version, user);
        get(self, url).await
    }

    /// Get Vec of shorted by current user links
    pub async fn get_my_shorted_links(&self) -> Result<Vec<ShortedLink>, BoticordError> {
        let url = api_url!("{}/links/get", &self.version,);
        post_with_response(self, url, Some(EmptyBody{})).await
    }

    /// Get Vec of shorted by current user links with the provided code
    ///
    /// # Arguments
    ///
    /// * `shortener_body` - Short information about a link, that we will search.
    ///
    pub async fn search_for_shorted_link(&self,
                                      shortener_body: ShortenerBody
    ) -> Result<Vec<ShortedLink>, BoticordError> {
        let url = api_url!("{}/links/get", &self.version,);
        post_with_response(self, url, Some(shortener_body)).await
    }

    /// Creates new shorted link
    ///
    /// # Arguments
    ///
    /// * `shortener_body` - Information about link we will create.
    ///
    pub async fn create_shorted_link(&self,
                                      shortener_body: ShortenerBody
    ) -> Result<ShortedLink, BoticordError> {
        let url = api_url!("{}/links/create", &self.version,);
        post_with_response(self, url, Some(shortener_body)).await
    }

    /// Deletes shorted link
    ///
    /// # Arguments
    ///
    /// * `shortener_body` - Information about link we will delete.
    ///
    pub async fn delete_shorted_link(&self,
                                     shortener_body: ShortenerBody
    ) -> Result<(), BoticordError> {
        let url = api_url!("{}/links/delete", &self.version);
        post(self, url, Some(shortener_body)).await
    }


    /// Post current bot's stats.
    /// # How to set BotStats? (example)
    ///
    /// # Arguments
    ///
    /// * `stats` - Stats that we will post
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use boticordrs::types::{BotStats};
    ///
    /// let stats = BotStats{servers: 2514, shards: 3, users: 338250};
    /// ```
    pub async fn post_bot_stats(&self, stats: BotStats) -> Result<(), BoticordError> {
        let url = api_url!("{}/stats", &self.version);
        post(self, url, Some(stats)).await
    }

    /// Post Server Stats Method.
    ///
    /// Remember, that only Boticord-Service Bots can do it in global,
    /// other will get an 403 error.
    /// (but it may works for custom bots, but you need a special API-token)
    ///
    ///
    /// # Arguments
    ///
    /// * `stats` - Stats that we will post
    ///
    pub async fn post_server_stats(&self, stats: ServerStats) -> Result<(), BoticordError> {
        let url = api_url!("{}/server ", &self.version);
        post(self, url, Some(stats)).await
    }
}

async fn request<T>(
    client: &BoticordClient,
    method: Method,
    url: String,
    data: Option<T>,
) -> Result<Response, BoticordError>
    where
        T: serde::Serialize + Sized,
{

    let mut req = client
        .client
        .request(method, &url)
        .header(AUTHORIZATION, &client.token);

    if let Some(data) = data {
        req = req.json(&data);
    }

    let resp = match req.send().await {
        Ok(resp) => resp,
        Err(e) => return Err(errors::from(e)),
    };
    match resp.status() {
        _ => resp.error_for_status().map_err(errors::from),
    }
}

async fn get<T>(client: &BoticordClient, url: String) -> Result<T, BoticordError>
    where
        T: serde::de::DeserializeOwned + Sized,
{
    let resp = request(client, Method::GET, url, None::<()>).await?;
    match resp.json().await {
        Ok(data) => Ok(data),
        Err(e) => Err(errors::from(e)),
    }
}


async fn post<T>(client: &BoticordClient,
                 url: String,
                 data: Option<T>) -> Result<(), BoticordError>
    where
        T: serde::Serialize + Sized,
{
    request(client, Method::POST, url, data).await?;
    Ok(())
}

async fn post_with_response<T, R>(client: &BoticordClient,
                    url: String,
                    data: Option<T>) -> Result<R, BoticordError>
    where
        T: serde::Serialize + Sized,
        R: serde::de::DeserializeOwned + Sized,
{
    let resp = request(client, Method::POST, url, data).await?;
    match resp.json().await {
        Ok(data) => Ok(data),
        Err(e) => Err(errors::from(e)),
    }
}