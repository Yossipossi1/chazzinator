use rusqlite::{Connection, Result};

/*
 * This file will create a database with appropriate tables
 * if not present and initialize the database if necessary.
 */

pub fn initialize_database() -> Result<()> {
    let connection = Connection::open("./database.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS user_time_roles (
            user_id INTEGER PRIMARY KEY,
            server_join_time INTEGER NOT NULL,
            visitor_added_time INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(())
}
