// fn area(width: u32, height: u32) -> u32 {
//     width * height
// } // inefficient definition of rectangle

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// } // the area function now has descriptive names that are more readable than the previous examples.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);
    println!(
        "The area of this rectangle is {} square pixels.\nUsing #[derive(Debug)]: {:#?}",
        rect1.area(),
        rect1
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "Using associated function inside an implementation block.\nSquare size: {:?}",
        sq
    );
    // println!(
    //     "The area of this rectangle is {} square pixels.\nUsing #[derive(Debug)]: {:#?}",
    //     area(&rect1),
    //     rect1
    // );
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // let rect1 = (30, 50);

    // println!(
    //     "Using a tuple, the area of the rectangle is {}",
    //     area2(rect1)
    // );
}
