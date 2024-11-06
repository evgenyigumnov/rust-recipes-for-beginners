fn process_result(value: Result<i32, String>) -> i32 {
    match value {
        Ok(x) => x * 2,
        Err(e) => {
            println!("Error: {}", e);
            0
        },
    }
}
fn main() {
    let result1 = process_result(Ok(5)); // 10
    let result2 = process_result(Err("Invalid value".to_string())); // Error: Invalid value
    println!("{}, {}", result1, result2); // Outputs: 10, 0
}