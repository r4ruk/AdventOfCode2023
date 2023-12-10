pub fn parse_line(input: &String, splitting_character: char) -> Vec<&str>{
    input.split(splitting_character).collect::<Vec<_>>()
}

pub fn parse_line_slice(input: &str, splitting_character: char) -> Vec<&str>{
    input.split(splitting_character).collect::<Vec<_>>()
}