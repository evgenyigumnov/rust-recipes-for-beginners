fn parse_number(s: &str) -> Result<u32, String> {
    s.parse::<u32>().map_err(|_| format!("Failed to parse '{}'", s))
}

fn main() {
    let parsed = parse_number("42"); // Ok(42)
    let failed_parse = parse_number("not_a_number"); // Err("Failed to parse 'not_a_number'")

    println!("{:?}, {:?}", parsed, failed_parse); // Outputs: Ok(42), Err("Failed to parse 'not_a_number'")
}