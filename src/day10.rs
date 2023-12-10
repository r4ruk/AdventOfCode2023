use crate::solver;

use geo::Contains;
use geo::Polygon;
use geo::Coord;

extern crate aoc_lib;
use aoc_lib::grid;
use aoc_lib::grid::manipulation::{self, Grid, Point};

pub struct SolverImpl;
impl solver::Solver for SolverImpl {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128 {
        let grid = grid::manipulation::Grid::<String>::new(inputs, manipulation::DataSeparatingCriteria::EachChar);
        let start_pnt = grid.find("S".to_string(), None);

        let mut point_before = start_pnt.clone();
        let mut next_point = grid.get(&Point{x_coord: start_pnt.x_coord-1, y_coord: start_pnt.y_coord, value: None}).clone();

        let mut amount = 0;
        loop {
            let new_next_point = get_next_point(point_before.clone(), &next_point, &grid);
            if "S" != next_point.value.as_deref().unwrap() {
                amount += 1;
                point_before = next_point.clone();
                next_point = new_next_point;
            } else {
                break;
            }
        }
        let result: i128;
        if amount % 2 == 0 {
            result = amount / 2 + 1;
        } else {
            result = (amount + 1) / 2
        }

        return result
    }

    fn solve_part2(&self, inputs: &Vec<String>) -> i128 {

        let grid = grid::manipulation::Grid::<String>::new(inputs, manipulation::DataSeparatingCriteria::EachChar);
        let startpnt = grid.find("S".to_string(), None);

        let mut point_before = startpnt.clone();
        let mut next_point = grid.get(&Point{x_coord: startpnt.x_coord-1,y_coord: startpnt.y_coord, value: None}).clone();

        let mut point_storage:Vec<Point<String>>= Vec::new();
        point_storage.push(next_point.clone());
        loop {
            let new_next_point = get_next_point(point_before.clone(), &next_point, &grid);
            if "S" != next_point.value.as_deref().unwrap() {
                point_storage.push(new_next_point.clone());
                point_before = next_point.clone();
                next_point = new_next_point;
            } else {
                break;
            }
        }

        let min_x =  point_storage.iter().min_by_key(|&e| &e.x_coord).unwrap().x_coord;
        let min_y =  point_storage.iter().min_by_key(|&e| &e.y_coord).unwrap().y_coord;
        let max_x =  point_storage.iter().max_by_key(|&e| &e.x_coord).unwrap().x_coord;
        let max_y =  point_storage.iter().max_by_key(|&e| &e.y_coord).unwrap().y_coord;

        let width = max_x - min_x;
        let height = max_y - min_y;

        let polygon_coords: Vec<(i32, i32)> = point_storage.iter().map(|element| (element.x_coord, element.y_coord)).collect();
        let polygon = Polygon::new(polygon_coords.into(), vec![]);

        let mut counter = 0;
        for x in 0..width {
            for y in 0..height {
                let grid_pnt = Coord { x, y };
                if polygon.contains(&grid_pnt){
                    counter += 1;
                }
            }
        }
        return counter
    }
}


fn solve_part3(inputs: &Vec<String>) -> i128 {

    let grid = grid::manipulation::Grid::<String>::new(inputs, manipulation::DataSeparatingCriteria::EachChar);
    let startpnt = grid.find("S".to_string(), None);

    let mut point_before = startpnt.clone();
    let mut next_point = grid.get(&Point{x_coord: startpnt.x_coord-1,y_coord: startpnt.y_coord, value: None}).clone();

    let mut point_storage:Vec<Point<String>>= Vec::new();
    point_storage.push(next_point.clone());
    loop {
        let new_next_point = get_next_point(point_before.clone(), &next_point, &grid);
        if "S" != next_point.value.as_deref().unwrap() {
            point_storage.push(new_next_point.clone());
            point_before = next_point.clone();
            next_point = new_next_point;
        } else {
            break;
        }
    }

    let min_x =  point_storage.iter().min_by_key(|&e| &e.x_coord).unwrap().x_coord;
    let min_y =  point_storage.iter().min_by_key(|&e| &e.y_coord).unwrap().y_coord;
    let max_x =  point_storage.iter().max_by_key(|&e| &e.x_coord).unwrap().x_coord;
    let max_y =  point_storage.iter().max_by_key(|&e| &e.y_coord).unwrap().y_coord;

    let width = max_x - min_x;
    let height = max_y - min_y;

    let polygon_coords: Vec<(i32, i32)> = point_storage.iter().map(|element| (element.x_coord, element.y_coord)).collect();
    let polygon = Polygon::new(polygon_coords.into(), vec![]);

    let mut counter = 0;
    for x in 0..width {
        for y in 0..height {
            let grid_pnt = Coord { x, y };
            if polygon.contains(&grid_pnt){
                counter += 1;
            }
        }
    }
    return counter
}



fn get_next_point(pnt_before: Point<String>,pnt_actual: &Point<String>,grid: &Grid<String>) -> Point<String>{
    let m = pnt_actual.value.as_deref().unwrap_or_default();
    match m {
        "|" => {
            if pnt_before.y_coord < pnt_actual.y_coord {
                return grid.get(&Point{y_coord: pnt_actual.y_coord+1, x_coord: pnt_actual.x_coord, value: None}).clone()
            } else {
                return grid.get(&Point{y_coord: pnt_actual.y_coord - 1, x_coord: pnt_actual.x_coord, value: None}).clone()
            }
        }
        "-" => {
            if pnt_before.x_coord < pnt_actual.x_coord {
                return grid.get(&Point{y_coord: pnt_actual.y_coord, x_coord: pnt_actual.x_coord + 1, value: None}).clone()
            } else {
                return grid.get(&Point{y_coord: pnt_actual.y_coord, x_coord: pnt_actual.x_coord - 1, value: None}).clone()
            }
        }
        "L" => { // north and east
            if pnt_before.x_coord > pnt_actual.x_coord {
                return grid.get(&Point{y_coord: pnt_actual.y_coord - 1 , x_coord: pnt_actual.x_coord, value: None}).clone()
            } else {
                return grid.get(&Point{y_coord: pnt_actual.y_coord, x_coord: pnt_actual.x_coord + 1, value: None}).clone()
            }
        }
        "J" => { // west and north
            if pnt_before.x_coord < pnt_actual.x_coord {
                return grid.get(&Point{y_coord: pnt_actual.y_coord - 1 , x_coord: pnt_actual.x_coord, value: None}).clone()
            } else {
                return grid.get(&Point{y_coord: pnt_actual.y_coord, x_coord: pnt_actual.x_coord - 1, value: None}).clone()
            }
        }
        "7" => { // west and south
            if pnt_before.x_coord < pnt_actual.x_coord {
                return grid.get(&Point{y_coord: pnt_actual.y_coord + 1 , x_coord: pnt_actual.x_coord, value: None}).clone()
            } else {
                return grid.get(&Point{y_coord: pnt_actual.y_coord, x_coord: pnt_actual.x_coord - 1, value: None}).clone()
            }
        }
        "F" => { // south and east
            if pnt_before.x_coord > pnt_actual.x_coord {
                return grid.get(&Point{y_coord: pnt_actual.y_coord + 1 , x_coord: pnt_actual.x_coord, value: None}).clone()
            } else {
                return grid.get(&Point{y_coord: pnt_actual.y_coord, x_coord: pnt_actual.x_coord + 1, value: None}).clone()
            }
        }
        _ => {
            // should never land here 
            return Point{x_coord: 0, y_coord: 0, value: Some("failed".to_string())};
        }
    }
}