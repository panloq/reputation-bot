mod database;
mod handler;
mod help;
mod rep;
mod repcheck;

use handler::Handler;
use serenity::{model::gateway::GatewayIntents, Client};
use std::{env, fs};

const DISCORD_TOKEN: &str = "your_discord_bot_token_here";

fn load_env() {
    if let Ok(content) = fs::read_to_string(".env") {
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                env::set_var(key.trim(), value.trim());
            }
        }
    }
}

#[tokio::main]
async fn main() {
    load_env();

    if let Err(e) = database::init_db() {
        eprintln!("Failed to initialize the database: {}", e);
        return;
    }

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    let mut client = Client::builder(DISCORD_TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create Discord client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
