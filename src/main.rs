mod database;
mod handler;
mod help;
mod rep;
mod repcheck;

use handler::Handler;
use serenity::{model::gateway::GatewayIntents, Client};

/// Paste your Discord bot token here.
const DISCORD_TOKEN: &str = "your_discord_bot_token_here";

#[tokio::main]
async fn main() {
    if DISCORD_TOKEN == "your_discord_bot_token_here" || DISCORD_TOKEN.is_empty() {
        eprintln!("Set DISCORD_TOKEN in src/main.rs before running the bot.");
        return;
    }

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
