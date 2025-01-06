#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Struct methods
impl Point {
    // Associated functions - static methods
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    // Methods
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn dist(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let mut p = Point::zero();
    println!("{:?}", p);

    p.move_to(3.0, 4.0);
    println!("{:?}", p);

    let distance = p.dist();
    println!("{}", distance);
}
