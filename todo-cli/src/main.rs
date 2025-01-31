use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs::{OpenOptions, write, read_to_string, create_dir_all};
use serde::{Serialize, Deserialize};
use std::io::Error;

#[derive(Subcommand)]
enum Keyword {
    #[command(about = "add a new task to your tasks")]
    Add {
        #[arg(help = "a brief description of a task")]
        description: String,  // positional argument after `add`
    },
    
    #[command(about = "remove a task by its ID")]
    Remove {
        #[arg(help = "ID of a task to remove, check using 'list command'")]
        id: u32,  // positional argument after `remove` and so on
    },
    
    #[command(about = "mark a task as complete")]
    Complete {
        #[arg(help = "ID of a task to mark as done, check using 'list command'")]
        id: u32,
    },
    
    #[command(about = "list all the tasks you have (useful to check IDs)")]
    List,
}

#[derive(Parser)]
#[command(
    name = "ToDo CLI",
    author = "Qqweweqewe",
    version = "1.0",
    about = "A ToDo list cli to keep your tasks in sight",
    long_about = "Use todo-cli [COMMAND] --help to see more"
)]
struct ToDo {
    #[command(subcommand)]
    command: Option<Keyword>,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: i32,
    description: String,
    done: bool
}

impl Task {
    fn mark_unmark(&mut self) {
        self.done = !self.done
    }
}

struct ToDoList {
    tasks: Vec<Task>
}

impl ToDoList {
    pub fn add_task(&mut self, desc: String) {
        let new_task = Task {
            id: self.tasks.len() as i32,
            description: desc,
            done: false
        };

        self.tasks.push(new_task) 
    }

    pub fn remove_task(&mut self, id: u32) {
        self.tasks.remove(id as usize);
    }

    pub fn complete_task(&mut self, id: u32) {
        let task_instance: Option<&mut Task> = self.tasks.get_mut(id as usize);
        match task_instance {
            Some(instance) => { instance.mark_unmark() },
            None => { panic!("No task with that id") }
        };
    }

    pub fn list_tasks(&self) {
        for instance in &self.tasks {
           println!("{}. {} [{}]", instance.id, instance.description, if instance.done { "x" } else { " " }); 
        }
    }

    pub fn save(&self, path: &PathBuf) {
        let json = serde_json::to_vec(&self.tasks).expect("failed to serialize data");
        write(path, json).expect("failed to save data");
    }

    pub fn load(path: &PathBuf) -> Result<Self, Error> {
        let data = read_to_string(path)?;
        let tasks = serde_json::from_str(&data)?; 
        Ok(Self { tasks })
    }

}

fn main() {
    let data_path: PathBuf = setup_config_dir().expect("something went wrong while loading data :(");

    let mut list = ToDoList::load(&data_path).expect("failed to load data");

    let args = ToDo::parse();

    match args.command {
        Some(Keyword::Add { description }) => {
            println!("Added: {}", description);
            list.add_task( description);
        }
        
        Some(Keyword::Remove { id }) => {
            println!("Removed ID: {}", id);
            list.remove_task(id);
        }
        
        Some(Keyword::List) => {
            list.list_tasks();
        }
        
        Some(Keyword::Complete { id }) => {
            println!("Task {} marked as done", id);
            list.complete_task(id);
        }
        
        None => {
            println!("Type 'todo-cli help' to see available commands")
        }
    }

    list.save(&data_path);
}

fn setup_config_dir() -> Result<PathBuf, Error> {
    let data_path: PathBuf = dirs::home_dir()
    .expect("Home directory not found")
    .join(".local/share")
    .join(env!("CARGO_PKG_NAME"));

    create_dir_all(&data_path)?;
    
    let file: PathBuf = data_path.join("data.json");

    if !file.exists() {
        write(&file, "[]")?;
    } else {
        OpenOptions::new()
            .read(true)
            .append(true)
            .open(&file)?;
    }

    Ok(file)
}
