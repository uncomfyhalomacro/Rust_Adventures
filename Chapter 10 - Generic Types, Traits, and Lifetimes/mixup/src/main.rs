struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<V, U> {
        Point {
            x: other.x,
            y: self.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    println!("p1.x {}, p1.y = {}", p1.x, p1.y);
    let p2 = Point { x: "Hello", y: 'c' };

    println!("p2.x {}, p2.y = {}", p2.x, p2.y);
    let p3 = p1.mixup(p2);
    println!("p3.x {}, p3.y = {}", p3.x, p3.y);
}
