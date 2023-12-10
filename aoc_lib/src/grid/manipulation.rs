
const ADJACENT_COORDS:[Point<i32>; 9]  = [
    Point{x_coord: -1, y_coord: -1, value:Some(0) },
    Point{x_coord: -1, y_coord: -1, value:Some(0) },
    Point{x_coord: 0, y_coord: -1, value:Some(0) },
    Point{x_coord: 1, y_coord: -1, value:Some(0) },
    Point{x_coord: -1, y_coord: 0, value:Some(0) },
    Point{x_coord: 1, y_coord: 0, value:Some(0) },
    Point{x_coord: -1, y_coord: 1, value:Some(0) },
    Point{x_coord: 0, y_coord: 1, value:Some(0) },
    Point{x_coord: 1, y_coord: 1, value:Some(0) }
];

pub struct Grid<T>{
    data: Vec<Vec<Point<T>>>,
}

#[derive(Clone)]
pub struct Point<T> {
    pub x_coord: i32,
    pub y_coord: i32,
    pub value: Option<T>
}

pub enum DataSeparatingCriteria {
    EachChar,
    ByWhitespace,
    ByChar(char)
}


impl<T> Grid<T> {
    fn is_valid_coordinate(&self, pnt: &Point<T>) -> bool {
        return pnt.x_coord >= 0
            && pnt.y_coord >= 0
            && pnt.y_coord < self.data.len() as i32
            && pnt.x_coord < self.data[0].len() as i32
    }
    pub fn get(&self, pnt: &Point<T>) -> &Point<T> {
        &self.data[pnt.y_coord as usize][pnt.x_coord as usize]
    }

    pub fn find (&self, search_value: T, pnt:Option<&Point<T>>) -> &Point<T> where T: PartialEq {
        match pnt {
            Some(start_point) => {
                for i in start_point.y_coord as usize..self.data.len() {
                    for j in start_point.x_coord as usize..self.data[i].len() {
                        let element: &Point<T> = &self.data[i][j];
                        if *element.value.as_ref().unwrap() == search_value {
                            return &element
                        }
                    }
                }
                return &Point{y_coord: 0, x_coord: 0, value: None }
            }
            None => {
                for i in 0..self.data.len() {
                    for j in 0..self.data[i].len() {
                        let element: &Point<T> = &self.data[i][j];
                        if *element.value.as_ref().unwrap() == search_value {
                            return &element
                        }
                    }
                }
                return &Point{y_coord: 0, x_coord: 0, value: None } 
            }
        }
    }
    pub fn width(&self) -> i32 {
        if self.data.len() > 0 {
            return self.data[0].len() as i32;
        }
        0
    }
    pub fn height(&self) -> i32 {
        return self.data.len() as i32;
    }

    pub fn new(lines: &Vec<String>, criteria: DataSeparatingCriteria) -> Grid<String>
    where T: Clone
    {
        match criteria {
            DataSeparatingCriteria::EachChar => {
                let data = Grid::<T>::build_grid_internal(lines, None);
                Grid{data}
            },
            DataSeparatingCriteria::ByChar(c) => {
                Grid{data: Grid::<T>::build_grid_internal(lines, Some(c))}
            },
            DataSeparatingCriteria::ByWhitespace => {
                Grid{data: Grid::<T>::build_grid_internal(lines, Some(' '))}
            }
        }
    }

    fn build_grid_internal(lines: &Vec<String>, separator: Option<char>) -> Vec<Vec<Point<String>>>
    {
        if separator.is_none() {
            let grid_chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

            let mut rows:Vec<Vec<Point<String>>> = vec![];
            for (y, chars) in grid_chars.iter().enumerate() {
                let mut char_cols: Vec<Point<String>> = vec![];
                for (x, char) in chars.iter().enumerate() {
                    char_cols.push(Point{
                        y_coord: y as i32,
                        x_coord: x as i32,
                        value: Some(char.to_string())
                    })
                }
                rows.push(char_cols);
            }
            return rows
        }

        let separator = separator.unwrap_or(' ');
        let mut grid_data = Vec::new();
        for (y, line) in lines.into_iter().enumerate() {
            let mut row = Vec::new();


            for (x, value_str) in line.split(separator).enumerate() {
                let point = Point {
                    x_coord: x as i32,
                    y_coord: y as i32,
                    value: Some(value_str.to_string())
                };
                row.push(point);
            }
            grid_data.push(row);
        }
        return grid_data
    }
}

impl<T: PartialEq + Clone> Point<T> {
    pub fn is_adjacent_to_value(&self, grid: Grid<T>, search_value: T) -> (bool, Vec<Point<T>>) {
        let mut adjacent_points:Vec<Point<T>>=vec![];

        for point in ADJACENT_COORDS {
            let temp_pt = Point {
                x_coord: self.x_coord + point.x_coord,
                y_coord: self.y_coord + point.y_coord,
                value: Some(search_value.clone())};

            // ensure valid coordinate
            if grid.is_valid_coordinate(&temp_pt) {
                // get actual grid character value
                let value = grid.get(&temp_pt).value.clone();

                if search_value == value.unwrap() {
                    adjacent_points.push(temp_pt)
                }
            }
        }
        return (!adjacent_points.is_empty(), adjacent_points)
    }
}