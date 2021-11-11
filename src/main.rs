mod config;

use config::{PREFIX, TOKEN};
use serde_json::value::Value;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use std::collections::HashMap;
use std::fs::File;

fn search_game(game: String) -> Option<i64> {
    let file = File::open("games.json").unwrap();
    let json: Value = serde_json::from_reader(file).unwrap();
    for app in json["response"]["apps"].as_array().unwrap() {
        if app["name"].as_str().unwrap() == game {
            return Some(app["appid"].as_i64().unwrap());
        }
    }
    None
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with(PREFIX) {
            let content = &*msg.content.replace(PREFIX, "");
            let mut cmd = content.split(" ").collect::<Vec<&str>>();
            match cmd[0] {
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
                "players" => {
                    cmd.remove(0);
                    let game = cmd.join(" ");
                    msg.reply(ctx,format!("there are {} degenerates playing {}\n https://tenor.com/view/capibara-gif-22235175",reqwest::get(format!("https://api.steampowered.com/ISteamUserStats/GetNumberOfCurrentPlayers/v1/?appid={}",search_game(game.clone()).unwrap()))        .await.unwrap()
                .json::<Value>()
                .await.unwrap()["response"]["player_count"].as_i64().unwrap(),game)).await;
                } //pog
                "price" => {
                    cmd.remove(0);
                    let game = cmd.join(" ");
                    let id = search_game(game).unwrap();
                    msg.reply(
                        ctx,
                        format!(
                            "It costs {}",
                            reqwest::get(format!(
                                "http://store.steampowered.com/api/appdetails?appids={}",
                                id
                            ))
                            .await
                            .unwrap()
                            .json::<Value>()
                            .await
                            .unwrap()[id.to_string()]["data"]["price_overview"]["final_formatted"]
                                .as_str()
                                .unwrap()
                        ),
                    )
                    .await;
                }

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
