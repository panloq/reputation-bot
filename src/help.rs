use crate::handler::command_name;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(ctx: &Context, msg: &Message) {
    let token = msg.content.split_whitespace().next().unwrap_or("");
    if command_name(token) != Some("help") {
        return;
    }

    if msg.guild_id.is_none() {
        let _ = msg
            .channel_id
            .say(&ctx.http, "This command only works in a server.")
            .await;
        return;
    }

    let help_text = "Available commands (prefixes: `.` `,` `!`):

- `.rep @user plus/minus` - Give or take a reputation point from a user.

- `.repcheck @user` - Check a user's reputation score.

- `.help` - Show this help message.

";

    let _ = msg.channel_id.say(&ctx.http, help_text).await;
}
