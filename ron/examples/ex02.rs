use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
}

fn main() {
    let point = Point { x: 1 };

    // Convert the Point to a JSON string.
    let serialized = ron::to_string(&point).unwrap();

    // Prints serialized = {"x":1}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = ron::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1}
    println!("deserialized = {:?}", deserialized);
}
