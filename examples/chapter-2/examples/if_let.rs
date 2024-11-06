fn print_if_some(value: Option<i32>) {
    if let Some(x) = value {
        println!("The value is: {}", x);
    }
}
fn main() {
    print_if_some(Some(42)); // Outputs: The value is: 42
}