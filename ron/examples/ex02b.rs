use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Point {
    x: i32,
}

fn main() {
    let serialized = "(x:1)";

    // Convert the JSON string back to a Point.
    let _deserialized: Point = ron::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1}
    // println!("deserialized = {:?}", deserialized);
}
