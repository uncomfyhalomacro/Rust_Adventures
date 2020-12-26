use std::io;

use io::Result;

fn main() {
    // let some_number = Some(5);
    // let some_string = Some("A string");
    // let absent_number: Option<i32> = None;
    //
    let x: i8 = 5;
    let mut input = String::new();
    println!("Input a number you wanna add:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");

    let input: Option<i8> = match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    };

    // if add(x, input).unwrap() == None {
    //     println!("You have not inputted any value!!")
    // } else {
    //     println!("Result: {}", add(x, input).unwrap().unwrap());
    // }

    if let Some(y) = input {
        println!("Result: {}", x + y);
    } else {
        println!("You have entered nothing or an invalid input!");
    }
}

// fn add(x: i8, num: Option<i8>) -> Result<Option<i8>> {
//     match num {
//         Some(i) => Ok(Some(i + x)),
//         None => Ok(None),
//     }
// }
