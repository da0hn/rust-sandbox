struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
        }
    }
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn scale(&self, factor: i32) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

fn main() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let p3 = p1.add(&p2);
    let p4 = p3.scale(3);
    println!("({}, {})", p4.x, p4.y);
}
