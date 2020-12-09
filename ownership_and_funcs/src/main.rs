fn main() {
    let s1 = String::from("hello");

    let s2 = s1.clone();
    
    takes_ownership(s2);
    
    let x = 5;
    let y = x;

    makes_copy(x);

    println!("Not affected because integers are weird! x = {}, y = {}", x, y);
}

fn takes_ownership(some_string: String) {
    println!("s2: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
