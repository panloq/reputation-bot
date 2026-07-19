use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, Result};
use std::path::Path;

const DB_PATH: &str = "data/users.db";

fn get_conn() -> Result<Connection> {
    if let Some(parent) = Path::new(DB_PATH).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    Connection::open(DB_PATH)
}

pub fn init_db() -> Result<()> {
    let conn = get_conn()?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS reputation (
            user_id  INTEGER NOT NULL PRIMARY KEY,
            rep      INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS rep_cooldown (
            user_id   INTEGER NOT NULL PRIMARY KEY,
            next_time TEXT NOT NULL
        );
        ",
    )?;

    Ok(())
}

pub fn get_rep(user_id: u64) -> Result<i32> {
    let conn = get_conn()?;

    let mut stmt = conn.prepare("SELECT rep FROM reputation WHERE user_id = ?1")?;

    let rep: i32 = stmt
        .query_row(params![user_id as i64], |row| row.get(0))
        .unwrap_or(0);

    Ok(rep)
}

pub fn add_rep(user_id: u64, change: i32) -> Result<i32> {
    let conn = get_conn()?;

    conn.execute(
        "INSERT OR IGNORE INTO reputation (user_id, rep) VALUES (?1, 0)",
        params![user_id as i64],
    )?;

    conn.execute(
        "UPDATE reputation SET rep = rep + ?2 WHERE user_id = ?1",
        params![user_id as i64, change],
    )?;

    get_rep(user_id)
}

pub fn get_cooldown(user_id: u64) -> Result<Option<DateTime<Utc>>> {
    let conn = get_conn()?;

    let mut stmt = conn.prepare("SELECT next_time FROM rep_cooldown WHERE user_id = ?1")?;

    let result = stmt.query_row(params![user_id as i64], |row| {
        let t: String = row.get(0)?;
        DateTime::parse_from_rfc3339(&t)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|_| rusqlite::Error::InvalidQuery)
    });

    match result {
        Ok(time) => Ok(Some(time)),
        Err(_) => Ok(None),
    }
}

pub fn set_cooldown(user_id: u64) -> Result<()> {
    let conn = get_conn()?;
    let next = Utc::now() + Duration::minutes(360);

    conn.execute(
        "INSERT OR REPLACE INTO rep_cooldown (user_id, next_time) VALUES (?1, ?2)",
        params![user_id as i64, next.to_rfc3339()],
    )?;

    Ok(())
}
