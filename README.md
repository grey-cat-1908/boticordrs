<p align="center">
<img width="520" src="https://media.discordapp.net/attachments/825242846616354821/939773822582808606/boticordrs_banner.png" alt="">
</p>

<p align="center">
  <b>
    The easiest way to use BotiCord API in Rust
    <span> · </span>
    <a href="https://docs.rs/boticordrs">Docs</a>
  </b>
</p>

<p align="center">
<a href="https://docs.rs/boticordrs"><img src="https://img.shields.io:/docsrs/boticordrs?style=flat-square" alt=""></a>
<a href="https://crates.io/crates/boticordrs"><img src="https://img.shields.io:/crates/d/boticordrs?style=flat-square" alt=""></a>
<a href="https://crates.io/crates/boticordrs"><img src="https://img.shields.io:/crates/v/boticordrs?style=flat-square" alt=""></a>
</p>


<h2>Usage</h2>

```toml
[dependencies]
boticordrs = "0.1.1"
```

<h2>Main Information: </h2>

boticordrs doesn't require using Serenity or Twilight. It makes working with the BotiCord API easier.
There are only some methods to use, but it's all what do we have in BotiCord API.

If you have any questions you can ask **[Marakarka#0575](https://boticord.top/profile/585766846268047370)** on [discord](https://boticord.top/discord).

<h3><em>What about BotiCord Webhooks?</em></h3>

BotiCord Webhooks are not implemented in boticordrs.

<h3><em>What about AutoLoop?</em></h3>

There is an example (with serenity). You can find it in the examples folder in our repository.

<h2>Examples: </h2>

<h3>Post Some Stats: </h3>

```rs
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
```
