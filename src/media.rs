use chrono::prelude::*;
use libsql::Connection;
use std::fmt;

#[derive(Debug)]
enum MediaType {
    Movie,
    Series,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MediaType::Movie => write!(f, "movie"),
            MediaType::Series => write!(f, "series"),
        }
    }
}

async fn insert_media(
    name: &str,
    media_type: MediaType,
    release: Option<DateTime<Local>>,
    conn: Connection,
) -> Connection {
    match release {
        Some(release) => {
            conn.execute(
                "INSERT INTO media (name, type, release) VALUES (?1, ?2, ?3)",
                [name, &media_type.to_string(), &release.to_rfc3339()],
            )
            .await
            .unwrap();
        }
        None => {
            conn.execute(
                "INSERT INTO media (name, type) VALUES (?1, ?2)",
                [name, &media_type.to_string()],
            )
            .await
            .unwrap();
        }
    }

    conn
}
