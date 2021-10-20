fn main() {
    println!("func {}", func_1());
    println!("func {}", func_2());
    println!("func {}", func_3());
}

fn func_1() -> i32 {
    1
}

fn func_2() -> i32 {
    return 2
}

fn func_3() -> i32 {
    return 3;
}