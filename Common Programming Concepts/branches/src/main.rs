fn main() {
    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let condition = true;
    let number2 = if condition { 5 } else { 6 };

    println!("The value of number2 is {}", number2);
}
