fn operation1() -> Result<i32, String> {
    Ok(10)
}

fn operation2() -> Result<Result<i32, String>, String> {
    Ok(operation1()) // returns Result<Result<i32, String>, String>
}

fn main() {
    let nested_result = operation2();

    let flattened: Result<i32, String> = nested_result.flatten();

    match flattened {
        Ok(value) => println!("Success with value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}