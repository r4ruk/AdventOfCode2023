
fn parse_string(input: String) -> i128 {
    match input.parse::<i128>() {
        Ok(parsed) => parsed,
        Err(err) => 0
    }
}