trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Modified print_area to accept a reference to a dyn Shape
fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    let rectangle = Rectangle {
        width: 5.0,
        height: 3.0,
    };
    print_area(&rectangle);

    let circle = Circle { radius: 2.5 };
    print_area(&circle);

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(rectangle), Box::new(circle)];

    for shape in shapes {
        print_area(&*shape);
    }
}
