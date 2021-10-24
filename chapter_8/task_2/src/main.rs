use std::io;

fn main() {
    let mut normal_string = String::new();

    println!("Write something:");
    io::stdin()
        .read_line(&mut normal_string)
        .expect("Failed to read line");

    let pig_string = pig_latin(&mut normal_string);
    println!("{}", pig_string);
}

fn pig_latin(normal_string : &mut String) -> String {
    let mut pig_string = String::new();
    let normal_words = normal_string.split_whitespace();

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for word in normal_words {
        let mut word_list = word.chars();
        let first_letter = match word_list.next() {
            Some(letter) => letter,
            None => {println!("Something went wrong"); continue},
        };

        if vowels.contains(&first_letter) {
            let pig_word = String::from(word_list.as_str());
            pig_string += &format!("{}-{}ay ", pig_word, first_letter);
        } else {
            pig_string += &format!("{}-{} ", word, "hay");
        }
    }

    pig_string
}