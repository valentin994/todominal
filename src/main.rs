mod args;
mod crud;

use args::TodominalArgs;
use crud::{get_all_todos, insert_todo, modify_todo, remove_todo};
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use crate::args::CrudCommand;
use ::clap::{Parser, ValueEnum};
use clap::Subcommand;
use directories::ProjectDirs;
use rusqlite::{Connection, Result};

//TODO move the db initialization to a build step
//TODO A way to update the todo
//TODO Change code to use Subcommands
//TODO A way to check on which operating system I am and how to store the todo
//TODO A way to reorganize the todos
//TODO Add colors for priorities
//TODO Add a config to be able to change the colors, specify file location for the sqlite
//TODO implement ratatui into this
//TODO try the palette crate for terminal color
//TODO enable deleting multiple todos with a flag
//TODO enable filtering on list

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    operation: Operation,
    #[arg(short, long)]
    text: Option<String>,
    #[arg(short, long, default_value = "medium")]
    priority: String,
    #[arg(short, long, default_value = None)]
    id: Option<i32>,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Subcommand)]
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
         priority  TEXT,
         date      TEXT
     )",
            (),
        )?;
        return Ok(db);
    }
}

fn main() {
    let conn = create_table().expect("Failed to connect to the DB");
    let args = TodominalArgs::parse();
    match args.crud {
        CrudCommand::Add(todo) => {
            match insert_todo(conn, todo.text, todo.priority) {
                Ok(added_todo) => println!("Added Todo: {}", added_todo),
                Err(err) => println!("Unable to add the todo {}", err),
            };
        }
        CrudCommand::Remove(todo) => {
            match remove_todo(conn, todo.id) {
                Ok(result) => println!("{result}"),
                Err(err) => println!("{err}"),
            };
        }
        CrudCommand::Modify(todo) => {
            match modify_todo(conn, todo.id, todo.text) {
                Ok(modified_todo) => println!("{modified_todo}"),
                Err(err) => println!("{err}"),
            };
        }
        CrudCommand::List => {
            match get_all_todos(conn) {
                Ok(()) => (),
                Err(err) => println!("{err}"),
            };
        }
    }
}
