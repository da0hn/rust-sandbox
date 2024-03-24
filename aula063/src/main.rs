struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

fn main() {
    let mut points = [
        Point { x: 3, y: 4 },
        Point { x: 6, y: 8 },
        Point { x: 2, y: 1 },
    ];

    for point in &mut points {
        point.translate(1, 2);
    }

    for point in points {
        println!("({}, {})", point.x, point.y);
    }
}