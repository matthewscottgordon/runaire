use rusqlite::{params, Connection, Result};

use std::fmt::Debug;
use std::path::Path;

fn populate_db_with_test_data<P: AsRef<Path> + Debug>(db_file_path: P) -> Result<()> {
    let conn = Connection::open(&db_file_path)?;
    let mut statement =
        conn.prepare("INSERT INTO todo_items (name, description, is_done) VALUES (?, ?, ?)")?;
    let test_data = vec![
        params!["Foo", "Foo the thing.", false],
        params!["Bar", "Apply barification.", false],
        params!["Baz", "Bazzy.", true],
    ];
    for row in test_data {
        statement.execute(row)?;
    }
    Ok(())
}

pub fn init_database<P: AsRef<Path> + Debug>(db_file_path: P) -> Result<()> {
    let conn = Connection::open(&db_file_path)?;

    conn.execute("DROP TABLE IF EXISTS todo_items;", params![])?;
    conn.execute(
        "CREATE TABLE todo_items (
             id   INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             description TEXT,
             is_done BOOL);",
        params![],
    )?;
    
    populate_db_with_test_data(&db_file_path)?;

    Ok(())
}
