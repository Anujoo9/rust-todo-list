#[derive(Debug)]
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

#[derive (Debug)]
struct TodoList{
    items: Vec<TodoItem>,
}

impl TodoList{
    fn new()-> Self{
        TodoList{
            items: Vec::new()
        }
    }
}

impl TodoList{
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

use std::io::{self, Write};

fn main(){
    let mut todo_list: TodoList = TodoList::new();

    loop{
        println!("1. Add item");
        println!("2. Remove item");
        println!("3. Mark as completed");
        println!("4. Display items");
        println!("5. Exit");

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
            5=> break,
            _ => println!("Invalid choice"),
        }
    }
}