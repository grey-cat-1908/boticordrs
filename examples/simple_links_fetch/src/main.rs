use boticordrs::{BoticordClient};
use boticordrs::types::ShortenerBody;

#[tokio::main]
async fn main() {
    let client = BoticordClient::new("your token".to_string(), 2).expect("failed client");

    match client.get_my_shorted_links().await {
        Ok(res) => {
            println!("First Link: {}", res[0].link)
        },
        Err(e) => eprintln!("{}", e),
    }
}