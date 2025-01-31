use rusqlite::{Connection, Result};

pub fn setup(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS file_metadata (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            modified_time TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn insert_file_metadata(conn: &Connection, path: &str, name: &str, modified_time: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO file_metadata (path, name, modified_time) VALUES (?1, ?2, ?3)",
        [path, name, modified_time],
    )?;
    Ok(())
}
