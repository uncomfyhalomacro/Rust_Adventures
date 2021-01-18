use std::{fs::File, io::ErrorKind};
fn main() {
    // panic!("crash and burn!")
    // let v = vec![1, 2, 3];

    // v[99];
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },

            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
