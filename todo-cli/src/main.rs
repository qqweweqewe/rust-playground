use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Keyword {
    // add a new task
    Add {
        // the value to add
        value: String,  // positional argument after `add`
    },
    
    // remove a task
    Remove {
        id: u32,  // positional argument after `remove` and so on
    },
    
    // complete a task
    Complete {
        id: u32,
    },
    
    List
}

#[derive(Parser)]
struct ToDo {
    #[command(subcommand)]
    command: Keyword,
}

fn main() {
    let args = ToDo::parse();
    match &args.command {
        Keyword::Add { value } => {
            println!("Added: {}", value);
        }
        Keyword::Remove { id } => {
            println!("Removed ID: {}", id);
        }
        Keyword::List => {
            println!("Imagine a task list here");
        }
        Keyword::Complete { id } => {
            println!("Task {} marked as done", id)
        }
    }
}
