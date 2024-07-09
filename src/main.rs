use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct TodoItem{
    id: u32,
    description: String,
    completed: bool,
}

impl TodoItem{
    fn new(id: u32, description: String)-> Self{
        TodoItem{
            id,
            description,
            completed: false,
        }
    }
}

#[derive (Debug, Serialize, Deserialize)]
struct TodoList{
    items: Vec<TodoItem>,
}

impl TodoList{
    fn new()-> Self{
        TodoList{
            items: Vec::new()
        }
    }

    fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>>{
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self.items)?;
        Ok(())
    }

    fn load_from_file(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>>{
        let file = File::open(filename)?;
        self.items = serde_json::from_reader(file)?;
        Ok(())
    }

    fn add_item(&mut self, description: String){
        let id = self.items.len() as u32 + 1;
        let item = TodoItem::new(id,description);
        self.items.push(item);
    }

    fn remove_item(&mut self, id:u32){
        self.items.retain(|item| item.id !=id);
    }

    fn mark_as_completed(&mut self, id: u32){
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id){
            item.completed = true;
        }
    }

    fn display_items(&self){
        for item in &self.items{
            println!("{:?}", item);
        }
    }
}


fn main(){
    let mut todo_list: TodoList = TodoList::new();
    // let filename: &str = "/Users/anujyadav/Anuj/programming/Rust/rust-todo-list-example/todo_items.json";

    let filename: &str = "todo_items.json";
    if Path::new(filename).exists(){
        if let Err(err) = todo_list.load_from_file(filename){
            eprintln!("Failed to load todo items from file: {}",err);
        }
    }else{
        println!("No exisiting todo items found. Starting with an empty list.");
    }

    loop{
        println!("1. Add item");
        println!("2. Remove item");
        println!("3. Mark as completed");
        println!("4. Display items");
        println!("5. Save items to file");
        println!("6. Exit");

        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please Enter a number");

        match choice {
            1 => {
                let mut description: String = String::new();
                print!("Enter description: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut description).expect("Failed to read line");
                let description = description.trim().to_string();
                todo_list.add_item(description);
            }
            2=>{
                let mut id = String::new();
                print!("Enter item ID to remove: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id : u32 = id.trim().parse().expect("Please enter a number");
                todo_list.remove_item(id);
            }
            3=> {
                let mut id = String::new();
                print!("Enter item ID to mark as completed. ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id:u32 = id.trim().parse().expect("Please enter a number");
                todo_list.mark_as_completed(id);
            }
            4=>{
                todo_list.display_items();
            }
            5=>{
                if let Err(err) = todo_list.save_to_file(filename){
                    eprintln!("Failed to save todo items to file: {}", err);
                }else{
                    println!("Todo items saved to file successfully");
                }
            }
            6=> break,
            _ => println!("Invalid choice"),
        }
    }
}