use std::fmt;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color({}, {}, {})", self.0, self.1, self.2)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {}, {})", self.0, self.1, self.2)
    }
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let white = Color(255, 255, 255);
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);

    println!("The color of the origin is {}", black);
    println!("The color of the sky is {}", blue);
    println!("The color of the grass is {}", green);

    let p1 = Point(10, 20, 30);
    let p2 = Point(-5, 15, 25);
    let p3 = Point(0, 0, 0);

    let distance = distance_between_points(&p1, &p2);
    println!("The distance between {} and {} is {}", p1, p2, distance);
}

fn distance_between_points(p1: &Point, p2: &Point) -> f64 {
    let x_diff = (p1.0 - p2.0).abs() as f64;
    let y_diff = (p1.1 - p2.1).abs() as f64;
    let z_diff = (p1.2 - p2.2).abs() as f64;

    (x_diff.powf(2.0) + y_diff.powf(2.0) + z_diff.powf(2.0)).sqrt()
}
