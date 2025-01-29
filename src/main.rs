use libsql::{Builder, Connection};

mod media;

async fn initialize_db() -> Connection {
    let db = Builder::new_local(":memory:").build().await.unwrap();
    let conn = db.connect().unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS media (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            type TEXT NOT NULL,
            release DATETIME
        )",
        (),
    )
    .await
    .unwrap();

    conn
}

#[tokio::main]
async fn main() {}
