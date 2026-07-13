use anyhow::{Context, Result};
use dialoguer::Confirm;
use rusqlite::{Connection, OpenFlags, OptionalExtension};
use std::io::ErrorKind;
use std::path::PathBuf;

// Do not change DB version.
pub const DB_VERSION: i64 = 12;
const OVERRIDES_SQL: &str = include_str!("../data/overrides.sql");

fn delete_db() -> anyhow::Result<()> {
    match std::fs::remove_file(db_path()?) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub struct OverrideEntry {
    pub key: String,
    pub override_type: String,
    pub kanji: String,
    pub reading: String,
    pub value: String,
    pub replace: i64,
}

pub fn db_version() -> Result<()> {
    let path = db_path()?;

    if !path.exists() {
        let answer = Confirm::new()
            .with_prompt("\nYou currently have no override database in your data directory. Would you like to generate one?")
            .interact()?;

        if answer {
            init_db();
            println!("Database generated.");
        } else {
            println!(
                "Source database version: {}\nLocal database will be generated when an override is needed, or when suzuri --update-db is ran.",
                DB_VERSION
            );
            std::process::exit(0);
        }
    }

    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    let current_version: i64 = conn
        .query_row(
            "SELECT value FROM meta WHERE key = 'db_version'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    println!(
        "\nLocal database version: {}\nSource database version: {}\n",
        current_version, DB_VERSION
    );

    if DB_VERSION > current_version {
        let version_lag: String;
        if DB_VERSION - current_version == 1 {
            version_lag = "1 version".to_string();
        } else {
            version_lag = format!("{} versions", DB_VERSION - current_version);
        }

        let answer = Confirm::new()
            .with_prompt(format!(
                "\nYour local database is {} behind. Would you like to update it?",
                version_lag
            ))
            .interact()?;

        if answer {
            init_db();
            println!("Database updated.");
            println!(
                "\nLocal database version: {}\nSource database version: {}\n",
                DB_VERSION, DB_VERSION
            );
        }
    }

    Ok(())
}

fn db_path() -> Result<PathBuf> {
    let mut path = dirs::data_dir().context("Could not determine data directory")?;
    path.push("suzuri");
    path.push("overrides.db");
    Ok(path)
}

pub fn empty_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS overrides (
            key           TEXT PRIMARY KEY,
            override_type  TEXT NOT NULL,
            kanji          TEXT NOT NULL,
            reading        TEXT NOT NULL,
            value     TEXT NOT NULL,
            replace        INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS meta (
            key   TEXT PRIMARY KEY,
            value INTEGER NOT NULL
        );",
    )?;

    Ok(conn)
}

pub fn init_db() -> Result<Connection> {
    let path = db_path()?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(path)?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS overrides (
            key           TEXT PRIMARY KEY,
            override_type  TEXT NOT NULL,
            kanji          TEXT NOT NULL,
            reading        TEXT NOT NULL,
            value     TEXT NOT NULL,
            replace        INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS meta (
            key   TEXT PRIMARY KEY,
            value INTEGER NOT NULL
        );",
    )?;

    let current_version: i64 = conn
        .query_row(
            "SELECT value FROM meta WHERE key = 'db_version'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if current_version < DB_VERSION {
        rebuild_overrides(&conn)?;
    }

    Ok(conn)
}

fn rebuild_overrides(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM overrides", [])?;
    conn.execute_batch(OVERRIDES_SQL)
        .context("Failed to execute overrides.sql")?;
    conn.execute(
        "INSERT INTO meta (key, value) VALUES ('db_version', ?1)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        [DB_VERSION],
    )?;
    Ok(())
}

pub fn get_override(conn: &Connection, key: &str) -> Result<Option<OverrideEntry>> {
    conn.query_row(
        "SELECT key, override_type, kanji, reading, value, replace FROM overrides WHERE key = ?1",
        [key],
        |row| {
            Ok(OverrideEntry {
                key: row.get(0)?,
                override_type: row.get(1)?,
                kanji: row.get(2)?,
                reading: row.get(3)?,
                value: row.get(4)?,
                replace: row.get(5)?,
            })
        },
    )
    .optional()
    .context("Failed to query override")
}
