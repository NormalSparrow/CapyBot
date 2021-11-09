mod config;

use config::{PREFIX, TOKEN};
use serde_json::value::Value;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use std::collections::HashMap;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with(PREFIX) {
            match &*msg.content.replace(PREFIX, "") {
                "bara" => {
                    msg.reply(ctx, "https://www.youtube.com/watch?v=APJZeNY6dKo")
                        .await;
                }
                "help" => {
                    msg.reply(ctx, "Commands:\n•bara \n•nekopara \n•price\n•TellHim")
                        .await;
                }
                "TellHim" => {
                    msg.reply(ctx, "Who asked Freddie").await;
                }
                "EyeBurner" => {
                    msg.reply(ctx, "https://ibb.co/3Frj6m4").await;
                }
                "nekopara" => {
                    msg.reply(ctx,format!("there are {} degenerates playing nekopara\n https://tenor.com/view/capibara-gif-22235175",reqwest::get("https://api.steampowered.com/ISteamUserStats/GetNumberOfCurrentPlayers/v1/?appid=1406990")        .await.unwrap()
                .json::<Value>()
                .await.unwrap()["response"]["player_count"].as_i64().unwrap())).await;
                } //pog
                "price" => {
                    msg.reply(
                        ctx,
                        format!(
                            "nekopara currently costs {} vbucks :pog:",
                            reqwest::get(
                                "http://store.steampowered.com/api/appdetails?appids=1406990"
                            )
                            .await
                            .unwrap()
                            .json::<Value>()
                            .await
                            .unwrap()["1406990"]["data"]["price_overview"]["final_formatted"]
                                .as_str()
                                .unwrap()
                        ),
                    )
                    .await;
                } //pog
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(TOKEN)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    client.start().await;
}
