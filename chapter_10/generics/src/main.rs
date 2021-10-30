fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let resutl = largest(&number_list);
    println!("The largest number is {}", resutl);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let resutl = largest(&char_list);
    println!("The largest char is {}", resutl);
}