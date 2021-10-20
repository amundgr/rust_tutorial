fn main() {
    normal_string();
    other_string();

    let s = String::from("String pointer");
    let s = ownership_fun(s);
    println!("{}", s);
}

fn normal_string() {
    // Needs to be know at compile time
    // Meaning needs to be "hard coded somwhere in code"
    let s = "Normal string";
    println!("{}", s);
}

fn other_string() {
    // Can be unknown during comlile time
    let s = String::from("Other string");
    println!("{}", s);
}

fn ownership_fun(some_string: String) -> String{
    // Takes ownership over values in heap
    // Unvalidates the old pointer
    // Needs to be returned to be used outside function scope after function execution
    println!("{}", some_string);
    some_string
}