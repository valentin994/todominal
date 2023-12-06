mod crud;

use crud::{get_all_todos, insert_todo, modify_todo, remove_todo};
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use ::clap::{Parser, ValueEnum};
use directories::ProjectDirs;
use rusqlite::{Connection, Result};
//TODO A way to update the todo
//TODO Change code to use Subcommands
//TODO A way to check on which operating system I am and how to store the todo
//TODO A way to reorganize the todos
//TODO Add colors for priorities
//TODO Add a config to be able to change the colors, specify file location for the sqlite
//TODO implement ratatui into this

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    operation: Operation,
    #[arg(short, long, default_value = " ")]
    text: String,
    #[arg(short, long, default_value = "medium")]
    priority: String,
    #[arg(short, long, default_value = None)]
    id: Option<i32>,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Operation {
    Add,
    Remove,
    Modify,
    Ls,
}

fn create_table() -> Result<Connection, rusqlite::Error> {
    let proj_dirs = ProjectDirs::from("", "", "todominal");
    let path = proj_dirs
        .clone()
        .unwrap()
        .config_dir()
        .join(Path::new("todominal.db3"));
    if path.exists() {
        let db = Connection::open(path)?;
        return Ok(db);
    } else {
        let _ = fs::create_dir_all(proj_dirs.clone().unwrap().config_dir());
        let db = Connection::open(path)?;
        let _ = db.execute(
            "CREATE TABLE todo (
         id INTEGER PRIMARY KEY AUTOINCREMENT,
         todo_text  TEXT NOT NULL,
         priority  TEXT
     )",
            (),
        )?;
        return Ok(db);
    }
}

fn main() {
    let conn = create_table().expect("Failed to connect to the DB");
    let args = Args::parse();
    match args.operation {
        Operation::Add => {
            match insert_todo(conn, args.text, args.priority) {
                Ok(()) => (),
                Err(err) => println!("Problems with adding the todo {}", err),
            };
        }
        Operation::Remove => {
            match remove_todo(conn, args.id.unwrap()) {
                Ok(()) => (),
                Err(err) => println!("{err}"),
            };
        }
        Operation::Modify => {
            match modify_todo(conn) {
                Ok(()) => (),
                Err(err) => println!("{err}"),
            };
        }
        Operation::Ls => {
            match get_all_todos(conn) {
                Ok(()) => (),
                Err(err) => println!("{err}"),
            };
        }
    }
}
