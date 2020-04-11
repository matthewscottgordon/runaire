use rusqlite::types::ToSql;
use rusqlite::{Connection, Result};

use crate::ToDoItem;

use std::fmt::Debug;
use std::path::Path;

fn populate_db_with_test_data<P: AsRef<Path> + Debug>(db_file_path: P) -> Result<()> {
    let conn = Connection::open(&db_file_path)?;
    let mut statement =
        conn.prepare("INSERT INTO todo_items (name, description, is_done) VALUES (?, ?, ?)")?;
    let test_data = vec![
        &[&"Foo" as &dyn ToSql, &"Foo the thing.", &false],
        &[&"Bar", &"Apply barification.", &false],
        &[&"Baz", &"Bazzy.", &true],
    ];
    for row in test_data {
        statement.execute(row)?;
    }
    Ok(())
}

pub fn init_database<P: AsRef<Path> + Debug>(db_file_path: P) -> Result<()> {
    let conn = Connection::open(&db_file_path)?;

    conn.execute("DROP TABLE IF EXISTS todo_items;", &[])?;
    conn.execute(
        "CREATE TABLE todo_items (
             id   INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             description TEXT,
             is_done BOOL);",
        &[],
    )?;

    populate_db_with_test_data(&db_file_path)?;

    Ok(())
}

pub fn get_all_todo_items(conn: &Connection) -> Result<Vec<ToDoItem>> {
    let mut statement = conn.prepare_cached(
        "SELECT name, description, is_done
             FROM todo_items;",
    )?;
    let result_iter = statement.query_map(&[], |row| {
        let name = row.get_checked("name")?;
        let description = row.get_checked("description")?;
        let is_done = row.get_checked("is_done")?;
        Ok(ToDoItem {
            name,
            description,
            is_done,
        })
    })?;
    result_iter.flatten().collect()
}
