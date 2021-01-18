use std::io;

fn main() {
    let mut user_input = String::new();
    // let mut user_input = String::from("vestergurkan");
    println!("Enter a word:");
    loop {
        let word = input(&mut user_input);
        if word.is_empty() {
            println!("Empty input! If you want to exit, press Ctrl+c.\nOtherwise, Try again:");
        } else {
            let pig = pig_latin(word);
            println!("Pigged: {}", pig);
            break;
        }
    }
}

fn pig_latin(s: &str) -> String {
    let vowels = String::from("aeiou");
    let first_letter: char = s.chars().next().unwrap_or_default();
    if vowels.contains(first_letter) {
        [s, "hay"].concat()
    } else {
        let new_str: &str = s
            .char_indices()
            .nth(1)
            .and_then(move |(i, _)| s.get(i..))
            .unwrap_or("");

        [new_str, &["-", &first_letter.to_string(), "ay"].concat()].concat()
    }
}

fn input(user_input: &mut String) -> &str {
    io::stdin()
        .read_line(user_input)
        .expect("Failed to read line");

    let user_input = user_input.trim();
    user_input
}
