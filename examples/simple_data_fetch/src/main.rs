use boticordrs::{BoticordClient};

#[tokio::main]
async fn main() {
    let client = BoticordClient::new("your token".to_string()).expect("failed client");

    let bot_id: String = "724663360934772797".to_string();

    match client.get_bot_info(bot_id).await {
        Ok(res) => {
            println!("Short Description: {}", res.information.short_description)
        },
        Err(e) => eprintln!("{}", e),
    }
}