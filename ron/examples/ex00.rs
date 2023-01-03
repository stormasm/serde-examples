use ron::value::{Number, Value};

fn main() {
    let array: [i32; 3] = [1, 2, 3];

    let ser = ron::to_string(&array).unwrap();
    assert_eq!(ser, "(1,2,3)");

    let de: [i32; 3] = ron::from_str(&ser).unwrap();
    assert_eq!(de, array);
    println!("{:?}", de);

    let value: Value = ron::from_str(&ser).unwrap();
    println!("{:?}", value);

    assert_eq!(
        value,
        Value::Seq(vec![
            Value::Number(Number::from(1)),
            Value::Number(Number::from(2)),
            Value::Number(Number::from(3)),
        ])
    );
}
