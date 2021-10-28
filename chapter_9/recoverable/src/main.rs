use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => File::create("hello.txt").expect("Problems creating file"),
            other_error => panic!("Problem opening the file: {:?}", other_error)
        },
    };

    let username = read_username_from_file().expect("Something went wrong");
    println!("{}", username);
    let username = read_username_from_file_compact().expect("Something went wrong");
    println!("{}", username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s  = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error)
    }
}

fn read_username_from_file_compact() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}