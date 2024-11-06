fn double_if_some(value: Option<i32>) -> Option<i32> {
    match value {
        Some(x) => Some(x * 2),
        None => None,
    }
}
fn main() {
    let result1 = double_if_some(Some(5)); // Some(10)
    let result2 = double_if_some(None); // None
    println!("{:?}, {:?}", result1, result2); // Outputs: Some(10), None
}