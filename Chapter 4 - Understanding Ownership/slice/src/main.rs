fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);

    println!("The first word is '{}'", word);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);

    println!("The first word is '{}'", word);

    let word = first_word(&my_string_literal);

    //s.clear(); // error!

    println!("The first word is '{}'", word);
    // let len = s.len();
    // let hello = &s[0..5]; // or let hello &s[..5]
    // let world = &s[6..12];

    // ending note from chapter about lists
    let a = [1, 2, 3, 4, 5];
    let slicei32 = &a[1..3];
    println!("Array slice result: {:?}", slicei32); // or to prettify -> println!("Array slice result: {:#?}", slicei32);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // or return &s[0..i];
        }
    }
    &s[..]
}
