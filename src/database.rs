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


pub fn record_exists(conn: &rusqlite::Connection, path: &str, name: &str) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM file_metadata WHERE path = ?1 AND name = ?2)")?;
    let exists: i64 = stmt.query_row([path, name], |row| row.get(0))?;
    Ok(exists != 0)
}

pub fn insert_file_metadata_if_not_exists(
    conn: &rusqlite::Connection,
    path: &str,
    name: &str,
    modified_time: &str,
) -> rusqlite::Result<()> {
    if !record_exists(conn, path, name)? {
        insert_file_metadata(conn, path, name, modified_time)?;
    }
    Ok(())
}

pub fn insert_file_metadata(conn: &Connection, path: &str, name: &str, modified_time: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO file_metadata (path, name, modified_time) VALUES (?1, ?2, ?3)",
        [path, name, modified_time],
    )?;
    Ok(())
}
