use crate::help;
use crate::rep;
use crate::repcheck;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct Handler;

/// Strips a leading `.`, `,`, or `!` prefix from a command token.
pub fn command_name(token: &str) -> Option<&str> {
    let name = token
        .strip_prefix('.')
        .or_else(|| token.strip_prefix(','))
        .or_else(|| token.strip_prefix('!'))?;

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let token = msg.content.split_whitespace().next().unwrap_or("");
        let Some(name) = command_name(token) else {
            return;
        };

        match name {
            "repcheck" => repcheck::run(&ctx, &msg).await,
            "help" => help::run(&ctx, &msg).await,
            "rep" | "r" | "reputation" => rep::run(&ctx, &msg).await,
            _ => {}
        }
    }
}
