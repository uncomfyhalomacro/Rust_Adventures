use std::println;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let c = vec![1, 2, 3];

    v.push(8);
    v.push(2);
    v.push(3);
    v.push(5);
    v.push(6);

    println!("v: {:?}, c: {:?}", v, c);

    let mut b = vec![1, 2, 3, 4, 5];

    let third: &i32 = &b[2];
    println!("The third element is {}", third);

    match b.get(7) {
        Some(third) => println!("The eight element is {}", third),
        None => println!("There is no eight element."),
    };

    // let does_not_exist = &v[100]; // panics
    let does_not_exist = v.get(100); // does not panic
    println!("{:?}", does_not_exist);
    for i in &v {
        println!("{}", i);
    }
    for u in &mut b {
        *u += 50;
        println!("{}", u);
    }
}
