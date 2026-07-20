# bot_rep

A Discord reputation bot written in Rust. Users can give or take reputation points with a cooldown, and check scores with simple text commands.

## Features

- Give or take reputation: `.rep @user +` / `.rep @user -`
- Check reputation: `.repcheck @user`
- View your own score and cooldown: `.rep`
- Help: `.help`
- Command prefixes: `.` `,` `!` (e.g. `,rep`, `!repcheck`)
- SQLite storage under `data/users.db` (created automatically)

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- A Discord bot token

## Setup

1. Clone the repository:

```bash
git clone <your-repo-url>
cd bot_rep
```

2. Open `src/main.rs` and paste your Discord bot token:

```rust
const DISCORD_TOKEN: &str = "your_discord_bot_token_here";
```

Optional: in `src/rep.rs`, set a user ID that skips the reputation cooldown:

```rust
const REP_BYPASS_USER_ID: Option<u64> = Some(123456789012345678);
```

3. In the [Discord Developer Portal](https://discord.com/developers/applications), enable these privileged intents for your bot:

- Message Content Intent

4. Invite the bot to your server with permissions to read and send messages.

5. Run the bot:

```bash
cargo run --release
```

## Commands

Prefixes: `.` `,` `!` — all commands work with any of them (e.g. `,rep`, `!help`).

| Command | Description |
|---------|-------------|
| `rep` / `r` / `reputation` | Show your reputation and cooldown |
| `rep @user +` or `plus` | Give +1 reputation |
| `rep @user -` or `minus` | Take −1 reputation |
| `repcheck @user` | Show another user's reputation |
| `help` | List available commands |

You can also reply to a message and use `.rep +` / `.rep -` without mentioning the user.

## Notes

- Reputation cooldown is **360 minutes** (6 hours) by default.
- Do **not** commit your bot token or the `data/` directory.
- The database is created automatically on first run at `data/users.db`.
