#![feature(proc_macro_hygiene, decl_macro)]

use rocket::config::{Config, Environment, Value};
use rocket::{get, routes};
use rocket_contrib;

use std::collections::HashMap;
use std::env;
use std::error;

mod database;

#[rocket_contrib::database("todo_db")]
struct DbConn(rusqlite::Connection);

#[get("/")]
fn index(conn: DbConn) -> &'static str {
    "Hello, world!"
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
        .launch();

    Ok(())
}
