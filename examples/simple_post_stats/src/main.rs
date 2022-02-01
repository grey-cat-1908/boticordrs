use boticordrs::{BoticordClient};
use boticordrs::types::{BotStats};

#[tokio::main]
async fn main() {
    let client = BoticordClient::new("your token".to_string()).expect("failed client");

    let stats = BotStats {servers: 2514, shards: 3, users: 338250};

    match client.post_bot_stats(stats).await {
        Ok(_) => {
            println!("Well Done!")
        },
        Err(e) => eprintln!("{}", e),
    }
}