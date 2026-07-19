use crate::database;
use crate::handler::command_name;
use chrono::Utc;
use serenity::model::prelude::*;
use serenity::prelude::*;

/// Optional: Discord user ID that skips the reputation cooldown.
/// Set to `Some(123456789012345678)` to enable, or `None` to disable.
const REP_BYPASS_USER_ID: Option<u64> = None;

fn has_rep_bypass(user_id: u64) -> bool {
    REP_BYPASS_USER_ID == Some(user_id)
}

pub async fn run(ctx: &Context, msg: &Message) {
    let content = msg.content.trim();
    let command = content.split_whitespace().next().unwrap_or("");

    if !matches!(command_name(command), Some("rep" | "r" | "reputation")) {
        return;
    }

    if msg.guild_id.is_none() {
        let _ = msg
            .channel_id
            .say(&ctx.http, "This command only works in a server.")
            .await;
        return;
    }

    let cleaned = content
        .strip_prefix(command)
        .unwrap_or("")
        .trim()
        .to_string();

    let target = if let Some(mentioned) = msg.mentions.first() {
        mentioned.id
    } else if let Some(referenced) = &msg.referenced_message {
        referenced.author.id
    } else if cleaned.contains('+') || cleaned.contains('-') {
        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                "You must mention a user or reply to their message.",
            )
            .await;
        return;
    } else {
        let rep = database::get_rep(msg.author.id.get()).unwrap_or(0);
        let cooldown_text = get_cooldown_text(msg.author.id.get()).await;
        let reply = format!(
            "Your reputation: **{}**. You can give reputation again in **{}**.",
            rep, cooldown_text
        );
        let _ = msg.channel_id.say(&ctx.http, reply).await;
        return;
    };

    if target == msg.author.id {
        let _ = msg
            .channel_id
            .say(&ctx.http, "You cannot change your own reputation.")
            .await;
        return;
    }

    let action = if cleaned.contains('-') { "-" } else { "+" };
    let change: i32 = if action == "+" { 1 } else { -1 };

    if !has_rep_bypass(msg.author.id.get()) {
        let now = Utc::now();
        if let Ok(Some(next)) = database::get_cooldown(msg.author.id.get()) {
            if next > now {
                let reply = format!(
                    "You can give reputation again in **{}**.",
                    format_remaining_time(next)
                );
                let _ = msg.channel_id.say(&ctx.http, reply).await;
                return;
            }
        }
    }

    match database::add_rep(target.get(), change) {
        Ok(new_rep) => {
            if !has_rep_bypass(msg.author.id.get()) {
                let _ = database::set_cooldown(msg.author.id.get());
            }
            let reply_text = if change > 0 {
                format!(
                    "+1 reputation for <@{}>, total **{}**.",
                    target.get(),
                    new_rep
                )
            } else {
                format!(
                    "-1 reputation for <@{}>, total **{}**.",
                    target.get(),
                    new_rep
                )
            };
            let _ = msg.channel_id.say(&ctx.http, reply_text).await;
        }
        Err(_) => {
            let _ = msg
                .channel_id
                .say(&ctx.http, "Database error, please try again.")
                .await;
        }
    }
}

async fn get_cooldown_text(user_id: u64) -> String {
    if has_rep_bypass(user_id) {
        return "right now".to_string();
    }

    match database::get_cooldown(user_id) {
        Ok(Some(next)) => {
            let now = Utc::now();
            if next > now {
                format_remaining_time(next)
            } else {
                "right now".to_string()
            }
        }
        _ => "right now".to_string(),
    }
}

fn format_remaining_time(next: chrono::DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = next.signed_duration_since(now);
    let minutes = diff.num_minutes();

    if minutes <= 0 {
        "right now".to_string()
    } else if minutes == 1 {
        "1 minute".to_string()
    } else {
        format!("{} minutes", minutes)
    }
}
