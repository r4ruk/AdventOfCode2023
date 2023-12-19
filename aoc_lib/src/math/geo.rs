#[derive(Clone, Copy)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}


pub fn shoelace_formula(points: &Vec<Coord>) -> i128 {
    let mut s1:i128 = 0;
    let mut s2:i128 = 0;
    for positions in points.windows(2) {
        s1 += positions[0].x as i128 * positions[1].y as i128;
        s2 += positions[1].x as i128 * positions[0].y as i128;
    }
    let area = (s1 - s2).abs() / 2;
    let perimeter = (points.len() - 1) as i128;
    area - perimeter / 2 + 1
}