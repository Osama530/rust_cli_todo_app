// use std::env;

// #[derive(Debug)]
// struct todo_iteam {
//     name: String,
//     compeleted: char,
// }

// impl todo_iteam {
//     fn new (name: String)-> todo_iteam {
//         todo_iteam{
//             name,
//             compeleted: '_',
//         }
//     }
// }

// struct Todo_List {
//     list: Vec<todo_iteam>,
// }

// impl Todo_List {
//     fn new ()->Todo_List{
//         Todo_List{
//             list:Vec::new(),
//         }
//     }

//     fn add_to_list (&mut self, name: String){
//         let item = todo_iteam::new(name);
//         self.list.push(item);
//     }
// }
// fn main() {
//     let arguments: Vec<String> = env::args().collect();
//     // read any command line arguments passed to it and then collect the values into a vector
//     let command = &arguments[1].clone();
//     let mut todo_lis = Todo_List::new();

//     let todo_list = Todo_List::new();

//     todo_lis.add_to_list("have break fast".to_string());
//     todo_lis.add_to_list("go to bead".to_string());

//     if command == "get" {
//         for item in todo_list.list {
//             println!("{:?}, {}", item.name, item.compeleted);
//         }

//     }
//     //println!("argument is {:?}", arguments);

// }

use std::env;
#[derive(Debug)]
struct Task {
    name: String,
    cmp_status: char,
}

impl Task {
    fn new(name: String) -> Task {
        Task {
            name,
            cmp_status: '_',
        }
    }
}
#[derive(Debug)]
struct Todo_List {
    list: Vec<Task>,
}
impl Todo_List {
    fn new() -> Todo_List {
        Todo_List { list: Vec::new() }
    }

    fn add_task(&mut self, name: String) {
        let item = Task::new(name);
        self.list.push(item);
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{} - {}, {}", index, item.name, item.cmp_status);
        }
    }

    fn maked_done(&mut self, index: usize) {
        if self.list[index].cmp_status == '_' {
            self.list[index].cmp_status = 'x'
        } else {
            self.list[index].cmp_status = '_'
        }
    }

    fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }
}

enum Commands {
    Get,
    Add(String),
    Done(usize),
    Remove(usize),
}
fn main() {
    //*************collecting data frome cli **************/
    let arrgs: Vec<String> = env::args().collect();
    // let command = &arrgs[1];
    let command = match arrgs[1].as_str() {
        "get" => Commands::Get,
        "add" => Commands::Add(arrgs[2].clone()),
        "done" => Commands::Done(arrgs[2].parse().unwrap()),
        "remove" => Commands::Remove(arrgs[2].parse().unwrap()),
        _ => panic!("you must enter proper command"),
    };
    //println!("{:?}",command);

    //***********************TODO App *************************/
    let mut list = Todo_List::new(); //creating a empty list

    list.add_task("visit to doctor".to_string()); //pushing task to a  list
    list.add_task("greb some breads krom market".to_string()); //pusing task to a  list
    list.maked_done(1);
    // let task_01 = Task::new("visit to doctor".to_string());
    // let mut list_01 = Todo_List::new();
    // list_01.list.push(task_01);

    // let mut list = Vec::new();
    // list.push(task_01);

    // for items in list.list {
    //     println!("{}, {}",items.name, items.cmp_status);
    // }

    // if command == "get" {
    //     list.print();
    // }
    // if command == "add" {
    //     let task = arrgs[2].clone();
    //     list.add_task(task);
    //     list.print();
    // }

    //println!("{:#?} ",list);

    match command {
        Commands::Get => list.print(),
        Commands::Add(task) => {
            list.add_task(task);
            list.print();
        }
        Commands::Done(index) => {
            list.maked_done(index);
            list.print();
        }
        Commands::Remove(index) => {
            list.remove(index);
            list.print();
        }
    }
}
