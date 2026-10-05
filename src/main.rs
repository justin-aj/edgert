// Lesson 4: impl on enums.
// Same idea as struct: impl attaches functions. Inside, use `match self`.

enum Shape {
    Circle(f64),
    Rect(f64, f64),
}

impl Shape {
    // &self: read only. match on self to see which variant it is.
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => 3.14 * r * r,
            Shape::Rect(w, h) => w * h,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "circle", // _ = ignore the data
            Shape::Rect(_, _) => "rect",
        }
    }
}

enum Light {
    Red,
    Green,
    Yellow,
}

impl Light {
    // &mut self: change which variant the value is.
    fn next(&mut self) {
        *self = match self {
            Light::Red => Light::Green,
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
        };
    }

    fn show(&self) {
        match self {
            Light::Red => println!("red"),
            Light::Green => println!("green"),
            Light::Yellow => println!("yellow"),
        }
    }
}

fn main() {
    let c = Shape::Circle(2.0);
    let r = Shape::Rect(3.0, 4.0);
    println!("{} area = {}", c.name(), c.area());
    println!("{} area = {}", r.name(), r.area());

    let mut light = Light::Red;
    light.show();
    light.next();
    light.show();
    light.next();
    light.show();
    light.next();
    light.show();

    // TODO 1: in `Shape::area`, delete the `Shape::Rect` arm. Read the error.
    // TODO 2: add a variant `Triangle(f64, f64)` to Shape. Run. Read the errors.
    //         Fix both `match` blocks. Compiler lists every place to update.
}
