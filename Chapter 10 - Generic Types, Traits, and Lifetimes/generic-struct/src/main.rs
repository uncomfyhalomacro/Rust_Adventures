#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U, // this won't work if the type parameter is just only one (e.g. T)
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let wont_work = Point { x: 5, y: 4.0 }; // works if there are a generic type parameter for it, hence, U
    println!("integer: {:?}", integer);
    println!("float: {:?}", float);
    println!("wont_work: {:?}", wont_work);
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("both_integer: {:?}", both_integer);
    println!("both_float: {:?}", both_float);
    println!("integer_and_float: {:?}", integer_and_float);
}
