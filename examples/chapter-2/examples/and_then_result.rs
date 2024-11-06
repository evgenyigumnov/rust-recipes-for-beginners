fn divide(numerator: u32, denominator: u32) -> Result<u32, String> {
    if denominator == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn parse_and_divide(numerator: &str, denominator: &str) -> Result<u32, String> {
    numerator.parse::<u32>()
        .map_err(|_| "Invalid numerator".to_string())
        .and_then(|num| denominator.parse::<u32>()
            .map_err(|_| "Invalid denominator".to_string())
            .and_then(|denom| divide(num, denom))
        )
}

fn main() {
    let result = parse_and_divide("10", "2"); // Ok(5)
    let division_by_zero = parse_and_divide("10", "0"); // Err("Division by zero")
    let invalid_input = parse_and_divide("ten", "2"); // Err("Invalid numerator")

    println!("{:?}, {:?}, {:?}", result, division_by_zero, invalid_input);
    // Outputs: Ok(5), Err("Division by zero"), Err("Invalid numerator")
}