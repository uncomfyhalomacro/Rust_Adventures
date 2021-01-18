use std::io;

fn main() {
    let mut user_input = String::new();
    // let mut user_input = String::from("vestergurkan");
    println!("Enter a word:");
    loop {
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let mut user_input = user_input.trim().to_string();
        if user_input.is_empty() {
            println!("Empty input! If you want to exit, press Ctrl+c.\nOtherwise, Try again:");
        } else {
            let pig = pig_latin(&mut user_input);
            println!("Pigged: {}", pig);
            break;
        }
    }
}

fn pig_latin(s: &mut String) -> String {
    let vowels = String::from("aeiou");
    let first_letter: char = s.chars().next().unwrap_or_default();
    let mut vector_chars: Vec<char> = Vec::new();
    for char in s.chars() {
        vector_chars.push(char);
    }
    if vowels.contains(first_letter) {
        s.push_str("hay");
        s.to_string()
    } else {
        let mut new_str = s
            .char_indices()
            .nth(1)
            .and_then(|(i, _)| s.get(i..))
            .unwrap_or("")
            .to_string();
        new_str.push('-');
        let mut new_substr: String = vector_chars[0].to_string();
        new_substr.push_str("ay");
        new_str.push_str(&new_substr);
        new_str
    }
}
