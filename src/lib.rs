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
    /// Constructs a new `Client`.
    pub fn new(token: String) -> Result<Self, BoticordError> {
        let client = ReqwestClient::builder().build().map_err(errors::from)?;
        Ok(BoticordClient { client, token })
    }

    /// Constructs a new `Client` with `ReqwestClient` specified by user.
    pub fn new_with_client(client: ReqwestClient, token: String) -> Self {
        BoticordClient { client, token }
    }

    /// Get information about a specific bot.
    pub async fn get_bot_info(&self, bot: String) -> Result<Bot, BoticordError> {
        let url = api_url!("/bot/{}", bot);
        get(self, url).await
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
        .header(AUTHORIZATION, &*client.token);

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
