<h1 align="center">BoticordRS</h1>

<p align="center">Rust crate for <a href="https://boticord.top/">Boticord</a> API</p>

<h3 align="center">
Project is currently in development.
</h3>

<h2>Main Information: </h2>

This crate doesn't require using Serenity or Twilight. It makes working with the Boticord API more user-friendly.
There are only some methods to use, but it's all what do we have in Boticord API.

If you have any questions you can `Marakarka#0575` on discord.

<h3><em>What about Boticord Webhooks?</em></h3>

In the future we will add some structs to work with. You can use `warp` or something like that to fetch them. (We will add an example)

<h3><em>What about AutoLoop?</e></h3>

We will add an example how to one (with `Serenity`, because `Twilight` users are usually Professionals)

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

<h2>ToDo: </h2>

<h3>Bots: </h3>

* [x] Get Bot's info method
* [x] Get Bot's comments method
* [x] Post Bot's stats method
* [x] Define Structures
  * [x] Bot
  * [x] Bot's Information
  * [x] Bot's Links
  * [x] Bot's attached server
* [x] Post bot's stats example

<h3>Servers: </h3>

* [x] Get Server's info method
* [x] Get Server's comments method
* [x] Post Server's stats method
* [x] Define Structures
    * [x] Server
    * [x] Information about the server
    * [x] Server's Links
    * [x] Server's stats

<h3>Users: </h3>

* [x] Get information about user method
* [x] Get user's comments
* [x] Get user's bots
* [x] Define Structures
    * [x] Information about user
    * [x] Single User's comment
    * [x] Single User's bot

<h3>Other: </h3>

* [ ] Add Webhooks-Using example (using `warp`)
* [ ] Define Structures
    * [x] Single Comment
    * [ ] Webhook Message - New Bot Bump
    * [ ] Webhook Message - Operation with comment