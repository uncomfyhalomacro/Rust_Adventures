fn main() {
   // let x = 5;

   // let x = x + 1;

   //let x = x * 2;
   // println!("The value of x is: {}", x);

   //let y = 2.0;

    // let z: f32 = 3.0;

    //println!("Test y = {}", y);

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    println!("s {} d {} p {} q {} r {}", sum, difference, product, quotient, remainder);
    
    let c = 'z';
    
    let e = 'ℤ';
    
    let heart_eyed_cat = '😻';
    
    println!("{} {} {}", c, e, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let a = [1, 2, 3, 4, 5];

    print!("test list {:#?}", a);

    let p: [i32; 5] = [1, 2, 3, 4, 5];

    println!("test p {:#?}", p);
    
    let o = 2.0; // f64
    
    let h: f32 = 3.0; // f32
    
    println!("f64 float o: {}, f32 float h: {}", o, h);

    another_function(5,124);

    let d = five();

    println!("From five() {} ", d);
    
    let r = five() + plus_one(5);

    println!("test r: {}", r);

    let s = plus_one(5);
    
    println!("Test var s: {}", s);
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    
    println!("The value of x and y are: {} and {}, respectively", x, y);

}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


