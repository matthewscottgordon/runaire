#![feature(proc_macro_hygiene, decl_macro)]

use rocket::config::{Config, Environment, Value};
use rocket::{get, routes};
use rocket_contrib;
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

use std::collections::HashMap;
use std::env;
use std::error;

mod database;

#[rocket_contrib::database("todo_db")]
struct DbConn(rusqlite::Connection);

#[derive(Serialize)]
pub struct ToDoItem {
    name: String,
    description: String,
    is_done: bool,
}

#[derive(Serialize)]
struct TemplateContext {
    todo_items: Vec<ToDoItem>,
}

#[get("/")]
fn index(conn: DbConn) -> Result<Template, rusqlite::Error> {
    let todo_items = database::get_all_todo_items(&*conn)?;
    Ok(Template::render("index", &TemplateContext { todo_items }))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    const DB_FILE_PATH_ENV_VAR: &'static str = "RUNAIRE_DATABASE_FILE";
    let db_file_path = match env::var(DB_FILE_PATH_ENV_VAR) {
        Ok(value) => value,
        Err(_) => {
            println!("{} must be set.", DB_FILE_PATH_ENV_VAR);
            std::process::exit(1)
        }
    };

    println!("{}", db_file_path);
    database::init_database(&db_file_path)?;

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(db_file_path));
    databases.insert("todo_db", Value::from(database_config));
    let config = Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![index])
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .launch();

    Ok(())
}
