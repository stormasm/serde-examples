use serde::Serialize;

#[derive(Serialize)]
struct Point {
    x: i32,
}

fn main() {
    let point = Point { x: 1 };

    // Convert the Point to a JSON string.
    ron::to_string(&point).unwrap();
}
