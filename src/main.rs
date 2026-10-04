// Lesson 2, step 2: enum.
// An enum value is exactly ONE of its variants.

use std::mem::size_of;

// Part A: plain enum. Variants carry no data.
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn name(d: Direction) -> &'static str {
    match d {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

// Part B: variants carry data. Each variant can hold different data.
enum Shape {
    Circle(f64),      // radius
    Rect(f64, f64),   // width, height
}

fn area(s: Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rect(w, h) => w * h,
    }
}

fn main() {
    println!("{}", name(Direction::Up));
    println!("{}", name(Direction::Left));

    println!("circle area = {}", area(Shape::Circle(2.0)));
    println!("rect area   = {}", area(Shape::Rect(3.0, 4.0)));

    println!("size Direction = {}", size_of::<Direction>());
    println!("size Shape     = {}", size_of::<Shape>());

    // TODO 1: delete the `Direction::Down` line in `name`. Run. Read the error.
    // TODO 2: guess size Direction and size Shape before you run. Then check.
}
