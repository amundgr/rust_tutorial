use std::{collections::HashMap, io};


fn main() {
    let mut register: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut command = String::new();
        println!("Command:");
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command_parsed = command.trim();

        match command_parsed {
            "add" => add_employee(&mut register),
            "dep" => print_departemnt(&mut register),
            "comp" => print_company(&mut register),
            _ => {println!("Unknown command {}, try agaen", command.trim()); continue},
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

    let name_vector = register.entry(department.trim().to_string()).or_insert(vec!());
    name_vector.push(name.trim().to_string());

}

fn print_departemnt(register : &mut HashMap<String, Vec<String>>) {
    let mut department = String::new();

    println!("Department to list:");
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");

    let department = department.trim();

    let people = match register.get(department) {
        Some(people) => people,
        None => {println!("Department not found"); return},
    };

    println!("{}: {:?}", department, people);
}

fn print_company(register : &mut HashMap<String, Vec<String>>) {
    println!("Company: {:?}", register);
}