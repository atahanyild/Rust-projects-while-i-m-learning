use crate::Materials::Structs::Item;
use std::io;

pub fn input(to_print: &str) -> String {
    let mut input = String::new();
    println!("{to_print}");
    io::stdin().read_line(&mut input);
    input
}

pub fn add_item(list: &mut Vec<Item>) {
    let task = input("Task to add: ");
    if let Some(_i) = list.iter().find(|x| *x.data == task.trim().to_string()) {
        println!("Task already existing.");
    } else {
        list.push(Item {
            data: task.trim().to_string(),
            completed: false,
        });
        println!("Succesfuly added.");
    }
}

pub fn delete_item(list: &mut Vec<Item>) {
    print_list(list);
    let task = input("Task to delete: ");
    if let Some(i) = list.iter().position(|a| a.data == task.trim()) {
        list.remove(i);
        println!("Succesfuly deleted.");
    } else {
        println!("Task not existing.");
    }
}

pub fn print_list(list: &mut Vec<Item>) {
    let mut c = 1;
    for i in list.iter() {
        println!("{} . {}", c, i.data);
        c += 1;
    }
    if list.len() == 0 {
        println!("your list is empty");
    }
}

pub fn complete_item(list: &mut Vec<Item>, comp_list: &mut Vec<Item>) {
    print_list(list);
    let task = input("Task to complate: ");
    if let Some(i) = list.iter().find(|a| a.data == task.trim()) {
        comp_list.push(Item {
            data: i.data.clone(),
            completed: true,
        });
        list.remove(list.iter().position(|x| x.data == task.trim()).unwrap());
        println!("Succesfuly complated. ");
    } else {
        println!("Task not existing.");
    }
}
