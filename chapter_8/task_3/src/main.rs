use std::{collections::HashMap, io};


fn main() {
    let mut register: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut command = String::new();
        println!("Command:");
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command_parsed = match command.trim() {
            Some(text) => text,
            None => {println!("Empty word, try again"); None},
        };

        match command_parsed {
            "add" => add_employee(&mut register),
            //"dep" => print_departemnt(&mut register),
            "comp" => print_company(&mut register),
            other => {println!("Unknown command {}, try agaen", command.as_str()); continue},
        }

    }
}

fn add_employee(register: &mut HashMap<String, Vec<String>>) {
    let mut name = String::new();
    let mut department = String::new();

    println!("Name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Department:");
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");

    let name_vector = register.entry(department.clone()).or_insert(vec!(name.clone()));
    name_vector.push(name.clone());

}

fn print_company(register : &mut HashMap<String, Vec<String>>) {
    let mut department = String::new();

    println!("Department to list:");
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");

    let people = match register.get(department.as_str()) {
        Some(people) => people,
        None => {println!("Department not found"); return},
    };

    println!("{}{:?}", department, people);
}