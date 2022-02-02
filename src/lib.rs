//! # boticordrs
//!
//! Crate for [Boticord](https://boticord.top/) API
//! ## Usage
//!
//! Add this to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! boticordrs = "0.0.1"
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
//!     let client = BoticordClient::new("your token".to_string()).expect("failed client");
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
#![doc(html_root_url = "https://docs.rs/boticordrs/0.0.1")]

use reqwest::header::AUTHORIZATION;
use reqwest::{Client as ReqwestClient, Response};
use reqwest::{Method};

macro_rules! api_url {
    ($e:expr) => {
        concat!("https://api.boticord.top/v1", $e)
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
}

impl BoticordClient {
    /// Constructs a new Client.
    pub fn new(token: String) -> Result<Self, BoticordError> {
        let client = ReqwestClient::builder().build().map_err(errors::from)?;
        Ok(BoticordClient { client, token })
    }

    /// Constructs a new Client with ReqwestClient specified by user.
    pub fn new_with_client(client: ReqwestClient, token: String) -> Self {
        BoticordClient { client, token }
    }

    /// Get information about a specific bot.
    pub async fn get_bot_info(&self, bot: String) -> Result<Bot, BoticordError> {
        let url = api_url!("/bot/{}", bot);
        get(self, url).await
    }

    /// Get information about a specific server.
    pub async fn get_server_info(&self, server: String) -> Result<Server, BoticordError> {
        let url = api_url!("/server/{}", server);
        get(self, url).await
    }


    /// Get Vec of bot's comments.
    pub async fn get_bot_comments(&self, bot: String) -> Result<Vec<SingleComment>, BoticordError> {
        let url = api_url!("/bot/{}/comments", bot);
        get(self, url).await
    }

    /// Get Vec of server's comments.
    pub async fn get_server_comments(&self, server: String) -> Result<Vec<SingleComment>, BoticordError> {
        let url = api_url!("/server/{}/comments", server);
        get(self, url).await
    }

    /// Post current bot's stats.
    /// # How to set BotStats? (example)
    ///
    /// ```no_run
    /// use boticordrs::types::{BotStats};
    ///
    /// let stats = BotStats{servers: 2514, shards: 3, users: 338250};
    /// ```
    pub async fn post_bot_stats(&self, stats: BotStats) -> Result<(), BoticordError> {
        let url = api_url!("/stats",);
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


async fn post<T>(client: &BoticordClient, url: String, data: Option<T>) -> Result<(), BoticordError>
    where
        T: serde::Serialize + Sized,
{
    request(client, Method::POST, url, data).await?;
    Ok(())
}