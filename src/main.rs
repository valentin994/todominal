use::clap::{Parser, ValueEnum};

//TODO Add a way to add a todo
//TODO A way to remove the todo
//TODO A way to check on which operating system I am and how to store the todo
//TODO A way to reorganize the todos

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    operation: Operation,
    #[arg(short, long)]
    text: String,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Operation {
    Add,
    Remove,
    Modify
}
fn main() {
    let args = Args::parse();
    println!("{:?}", args)

}
