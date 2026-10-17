use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo{
    pub id:u32,
    pub title: String,
    pub completed: bool,
}

impl Todo{
    //constructors
    pub fn new(id: u32, title:String) -> Self{
        Todo{
            in,
            tttle,
            completed: false,
        }
    }
    // methods
    pub fn display(&self){
        let status = 
            if self.completed {"[x]"}
            else{"[ ]"};
            println!("{} {} - {}", status, self.id,self.title);
    }
}
