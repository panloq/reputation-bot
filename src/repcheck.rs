use crate::database;
use crate::handler::command_name;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(ctx: &Context, msg: &Message) {
    let command = msg.content.split_whitespace().next().unwrap_or("");

    if command_name(command) != Some("repcheck") {
        return;
    }

    if msg.guild_id.is_none() {
        let _ = msg
            .channel_id
            .say(&ctx.http, "This command only works in a server.")
            .await;
        return;
    }

    if msg.mentions.is_empty() {
        let _ = msg
            .channel_id
            .say(&ctx.http, "Usage: .repcheck @user")
            .await;
        return;
    }

    let target = msg.mentions[0].id;
    let rep = database::get_rep(target.get()).unwrap_or(0);

    let reply = format!(
        "User <@{}> currently has **{}** reputation points.",
        target.get(),
        rep
    );

    let _ = msg.channel_id.say(&ctx.http, reply).await;
}
