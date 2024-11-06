fn main() {
    let valid_age: Option<u32> = Some(30);
    let missing_age: Option<u32> = None;

    println!("{}", valid_age.unwrap_or(20)); // Outputs: 30
    println!("{}", missing_age.unwrap_or(20)); // Outputs: 20

    let error_result: Result<u32, &str> = Err("Failed");
    let success_result: Result<u32, &str> = Ok(100);

    println!("{}", error_result.unwrap_or_else(|_| 0)); // Outputs: 0
    println!("{}", success_result.unwrap_or_else(|_| 0)); // Outputs: 100
}
