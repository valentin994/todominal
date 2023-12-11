use clap::{Args, Parser, Subcommand};
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TodominalArgs {
    #[clap(subcommand)]
    pub crud: CrudCommand,
    //create a config subcommand
}

#[derive(Debug, Subcommand)]
pub enum CrudCommand {
    /// Add a new todo to the database
    #[clap(alias = "a")]
    Add(AddTodo),
    /// Remove a todo from the database
    #[clap(alias = "rm")]
    Remove(RemoveTodo),
    /// Modify a todo from the database
    #[clap(alias = "mod")]
    Modify(ModifyTodo),
    /// List all todos
    #[clap(alias = "ls")]
    List(ListTodo),
}

#[derive(Args, Debug)]
pub struct AddTodo {
    /// The todo text
    pub text: String,

    /// The priority of the todo
    #[arg(short, long, default_value = "medium")]
    pub priority: String,
}

#[derive(Args, Debug)]
pub struct ListTodo {
    /// The priority of the todo
    #[arg(short, long)]
    pub priority: Option<String>,
}


#[derive(Args, Debug)]
pub struct RemoveTodo {
    /// The id of the todo
    pub id: i32,
}

#[derive(Args, Debug)]
pub struct ModifyTodo {
    /// The id of the todo
    pub id: i32,

    /// The text to update the todo with
    pub text: String,
}
