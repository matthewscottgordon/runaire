#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, ignite, routes};

use std::env;
use std::error;

mod database;

#[get("/")]
fn index() -> &'static str {
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

    ignite().mount("/", routes![index]).launch();

    Ok(())
}
