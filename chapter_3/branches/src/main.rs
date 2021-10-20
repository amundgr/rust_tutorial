fn main() {
    let number = test();

    println!("{}", number);
}

fn test() -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10{
            break counter;
        }
    }
}
