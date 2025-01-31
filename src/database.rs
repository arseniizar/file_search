use rusqlite::{Connection, Result};

pub fn setup() -> Result<Connection> {
    let conn = Connection::open("file_index.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS file_metadata (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL,
            modified_time TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn insert_file_metadata(path: &str, modified_time: &str) -> Result<()> {
    let conn = setup()?;
    conn.execute(
        "INSERT INTO file_metadata (path, modified_time) VALUES (?1, ?2)",
        [path, modified_time],
    )?;
    Ok(())
}
