
pub fn parse_string(input: String) -> i128 {
    match input.parse::<i128>() {
        Ok(parsed) => parsed,
        Err(_) => 0
    }
}
pub fn parse_string_ref(input: &str) -> i128 {
    match input.parse::<i128>() {
        Ok(parsed) => parsed,
        Err(_) => 0
    }
}

pub fn get_smallest_from_vec(input: &Vec<i128>) -> i128 {
    let smallest = input.iter().cloned().min();

    match smallest {
        Some(min_value) => min_value,
        None => 0
    }
}
pub fn get_biggest_from_vec(input: &Vec<i128>) -> i128 {
    let biggest = input.iter().cloned().max();

    match biggest {
        Some(max_value) => max_value,
        None => 0
    }
}