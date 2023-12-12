mod args;
mod crud;

use args::TodominalArgs;
use crud::{create_table, get_all_todos, insert_todo, modify_todo, remove_todo};

use crate::args::CrudCommand;
use ::clap::Parser;

//TODO A way to reorganize the todos
//TODO Add colors for priorities
//TODO Add a config to be able to change the colors, specify file location for the sqlite
//TODO implement ratatui into this
//TODO try the palette crate for terminal color
//TODO enable deleting multiple todos with a flag
//TODO enable filtering on list
//TODO filter by date
//TODO ascending and descending

fn main() {
    let conn = create_table().unwrap();
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
        CrudCommand::List(params) => {
            match get_all_todos(conn, params.priority) {
                Ok(()) => (),
                Err(err) => println!("{err}"),
            };
        }
    }
}
