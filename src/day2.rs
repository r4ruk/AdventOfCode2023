use crate::{num_util, solver};

pub struct SolverImpl;

impl solver::Solver for SolverImpl {
    fn solve_part1(&self, input: &Vec<String>) -> i128 {
       let mut res:i128 = 0;

        for row in input {
            // split at :
            let play: Vec<&str> = row.split(':').collect();

            // convert game number to integer
            let game_num_str: Vec<&str> = play[0].split_whitespace().collect();
            let game_num = num_util::parse_string_ref(game_num_str[1]);

            // collect all plays in game
            let plays: Vec<&str>  = play[1].split(';').collect();

            // if game is valid add to result
            if Self::is_valid_game(plays) {
                res += game_num;
            }
        }
        return res
    }

    fn solve_part2(&self, input: &Vec<String>) -> i128 {
        let mut res:i128 = 0;

        for row in input {

            // split at :
            let play: Vec<&str> = row.split(':').collect();
            // collect all plays in game
            let plays: Vec<&str>  = play[1].split(';').collect();

            // Add power of game to result
            res += Self::get_power_of_game(plays);
        }
        return res
    }
}

impl SolverImpl {
    fn is_valid_game(plays: Vec<&str>) -> bool {
        const MAX_RED:i128 = 12;
        const MAX_GREEN:i128 = 13;
        const MAX_BLUE:i128 = 14;

        // its a valid game if not any color number in a game is bigger than the consts given
        let mut is_valid_game = true;

        for p in plays {

            // get each colors value
            let colors_values: Vec<&str> = p.split(',').collect();

            // break if the play before was invalid (1 play invalid -> game invalid)
            if !is_valid_game {
                break;
            }
            // run over all colors values and check for validity
            for color in colors_values {
                let color_val_split: Vec<&str> = color.split_whitespace().collect();
                if color_val_split[1] == "green" {
                    if (num_util::parse_string_ref(color_val_split[0])) > MAX_GREEN {
                        is_valid_game = false;
                        break;
                    }
                } else if color_val_split[1] == "red" {
                    if (num_util::parse_string_ref(color_val_split[0])) > MAX_RED {
                        is_valid_game = false;
                        break;
                    }
                } else if color_val_split[1] == "blue" {
                    if (num_util::parse_string_ref(color_val_split[0])) > MAX_BLUE {
                        is_valid_game = false;
                        break;
                    }
                }
            }
        }
        is_valid_game
    }

    fn get_power_of_game(plays: Vec<&str>) -> i128 {
        let mut red_set: Vec<i128> = vec![];
        let mut blue_set: Vec<i128> = vec![];
        let mut green_set: Vec<i128> = vec![];

        for p in plays {
            // get each colors value
            let colors_values: Vec<&str> = p.split(',').collect();
            for color in colors_values {
                // split at whitespace to get number separated from color
                let color_val_split: Vec<&str> = color.split_whitespace().collect();

                // add value to corresponding color set
                if color_val_split[1] == "green" {
                    green_set.push(num_util::parse_string_ref(color_val_split[0]))
                } else if color_val_split[1] == "red"{
                    red_set.push(num_util::parse_string_ref(color_val_split[0]))

                } else if color_val_split[1] == "blue"{
                    blue_set.push(num_util::parse_string_ref(color_val_split[0]))
                }
            }
        }

        // figure out biggest of each color and multiply
        let max_green = num_util::get_biggest_from_vec(&green_set);
        let max_blue = num_util::get_biggest_from_vec(&blue_set);
        let max_red = num_util::get_biggest_from_vec(&red_set);
        return (max_green * max_blue * max_red);
    }

}


