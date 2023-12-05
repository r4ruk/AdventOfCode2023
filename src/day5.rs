use crate::{num_util, solver};

struct Function {
    tuples: Vec<(i128, i128, i128)>,
}

impl Function {
    fn new(s: &mut Vec<String>) -> Self {
        let temp_fn: Vec<_> = s.iter().map(|item| {
            let tmp: Vec<&str> = item.split_whitespace().collect();
            let destination = num_util::parse_string_ref(tmp[0]);
            let source = num_util::parse_string_ref(tmp[1]);
            let size = num_util::parse_string_ref(tmp[2]);
            (destination, source, size)
        }).collect();

        Function {tuples: temp_fn }
    }

    // function used for part 1
    fn apply_one(&self, x: i128) -> i128 {
        for (dst, src, sz) in &self.tuples {
            if *src <= x && x < *src + *sz {
                return x + dst - src;
            }
        }
        x
    }

    fn apply_range(&self, mut ranges: Vec<(i128, i128)>) -> Vec<(i128, i128)> {
        // range magic for part 2
        let mut new_ranges = Vec::new();
        for (dest, source, size) in &self.tuples {
            // run over each actual function
            let src_end = source + size;
            let mut new_ranges_tmp = Vec::new();
            // while ranges are existent to cover by functions
            while let Some((start, end)) = ranges.pop() {
                let before = (start, end.min(*source));
                let inter = (start.max(*source), src_end.min(end));
                let after = (src_end.max(start), end);
                // if range is not fully covered by range and is before
                if before.1 > before.0 {
                    new_ranges_tmp.push(before);
                }
                // if range intersects push into new range for next functions run
                if inter.1 > inter.0 {
                    new_ranges.push((inter.0 - source + dest, inter.1 - source + dest));
                }
                // if range is after actual functions range push into new ranges temp cause it
                // could still be covered by this function
                if after.1 > after.0 {
                    new_ranges_tmp.push(after);
                }
            }
            // append to ranges so get tested on this function
            ranges = new_ranges_tmp;
        }
        // push remaining ranges to new ranges as these are 1:1 mapped onto next function
        new_ranges.append(&mut ranges);
        // return new ranges for next run over function
        new_ranges
    }
}
pub struct SolverImpl;
impl solver::Solver for SolverImpl {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128 {

        let (seed_str, others) = inputs.split_at(1);
        let seed_txt: Vec<_> = seed_str[0].split(":").collect();

        // number parsing magic for seeds
        let seed: Vec<i128> = Self::parse_seeds(seed_txt);
        let functions: Vec<Function> = Self::build_functions(others);

        let mut num_storage: Vec<i128> = vec![];

        // for every seed number
        for mut x in seed {
            // apply functions in order
            for f in &functions {
                x = f.apply_one(x)
            }
            // push final destination number into set
            num_storage.push(x);
        }
        // return smallest destination number
        return *num_storage.iter().min().unwrap();
    }
    fn solve_part2(&self, inputs: &Vec<String>) -> i128 {
        // splitting inputs into seed values and functions
        let (seed_str,others) = inputs.split_at(1);
        let seed_txt: Vec<_> = seed_str[0].split(":").collect();

        // parsing seed number stuff
        let seed: Vec<i128> = Self::parse_seeds(seed_txt);

        let functions = Self::build_functions(others);

        let ranges: Vec<(i128, i128)> = Self::build_ranges(seed);
        let mut range_storage: Vec<(i128, i128)> = vec![];

        // loop over every range
        for (start, size) in ranges {
            let mut range = vec![(start, start + size)];
            for f in &functions {
                range = f.apply_range(range);
            }
            // push the smallest from actual range
            range_storage.push(*range.iter().min_by_key(|item| item.1).unwrap())
        }
        // take smallest
        range_storage.iter().min().unwrap().1
    }
}

impl SolverImpl {
    fn build_functions(others: &[String]) -> Vec<Function> {
        // building functions depending on input informations
        let mut functions: Vec<Function> = vec![];
        let mut index = 2;
        let mut temp: Vec<String> = vec![];
        while index < others.len() {
            if others[index] != "" {
                temp.push(others[index].to_string());
                index += 1;
            } else {
                let func = Function::new(&mut temp);
                functions.push(func);
                index += 2;
                temp.clear();
            }
        }
        functions
    }

    fn build_ranges(seed: Vec<i128>) -> Vec<(i128, i128)>{
        let mut index = 0;
        let mut ranges: Vec<(i128, i128)> = vec![];
        while index < seed.len() {
            ranges.push((seed[index], seed[index] + seed[index+1]));
            index += 2;
        }
        return ranges
    }

    fn parse_seeds(seed_txt: Vec<&str>) -> Vec<i128> {
        seed_txt[1].split_whitespace().collect::<Vec<_>>().iter().map(|x| num_util::parse_string_ref(x)).collect()
    }
}


// use crate::{num_util, solver};
// use rayon::prelude::*;
//
// struct ConversionMap {
//     destination: i128,
//     source: i128,
//     size: i128
// }
// struct Range {
//     from: i128,
//     to: i128,
// }
//
// pub struct SolverImpl;
// impl solver::Solver for SolverImpl {
//     fn solve_part1(&self, input: &Vec<String>) -> i128 {
//
//         let conversions = Self::build_conversions(&input);
//
//          let seeds: Vec<&str> = input[0].split_whitespace().collect();
//
//          let mut location_storage: Vec<i128> = vec![];
//
//          for seed in &seeds[1..seeds.len()] {
//              let mut temp_num = num_util::parse_string_ref(seed);
//              for map in &conversions {
//                  temp_num = solve_conversion(&map, temp_num)
//              }
//              location_storage.push(temp_num)
//          }
//         let res = location_storage.iter().cloned().min().unwrap();
//         return res
//     }
//
//     fn solve_part2(&self, input: &Vec<String>) -> i128 {
//         let conversions = Self::build_conversions(&input);
//
//         let seed_data: Vec<&str> = input[0].split(':').collect::<Vec<_>>()[1].split_whitespace().collect();
//
//         let mut index = 0;
//         let mut ranges: Vec<Range> = vec![];
//         while index < seed_data.len() {
//             let start = num_util::parse_string_ref(seed_data[index]);
//             ranges.push(Range{
//                 from: start,
//                 to: start + num_util::parse_string_ref(seed_data[index+1])
//             });
//             index += 2;
//         }
//         let numbers_to_try: Vec<_> = ranges.par_iter().flat_map(|range| range.from..=range.to).collect();
//         let amount_to_try = numbers_to_try.len();
//
//         println!("numbers to try amount: {amount_to_try}");
//
//         let smallest = numbers_to_try
//             .into_par_iter()
//             .map(|number| process_number(number, &conversions))
//             .reduce(|| i128::MAX, |a, b| if a < b {a} else {b});
//
//         smallest
//     }
// }
//
// impl SolverImpl {
//     fn build_conversions(input: &&Vec<String>) -> Vec<Vec<ConversionMap>> {
//         let seed2soil = &input[3..17];
//         let soil2fertilizer = &input[19..30];
//         let fertilizer2water = &input[33..66];
//         let water2light = &input[68..98];
//         let light2temperature = &input[100..143];
//         let temperature2humidity = &input[145..191];
//         let humidity2location = &input[193..213];
//
//         let mut conversions: Vec<Vec<ConversionMap>> = vec![];
//         conversions.push(get_conversionmap_for_lines(seed2soil));
//         conversions.push(get_conversionmap_for_lines(soil2fertilizer));
//         conversions.push(get_conversionmap_for_lines(fertilizer2water));
//         conversions.push(get_conversionmap_for_lines(water2light));
//         conversions.push(get_conversionmap_for_lines(light2temperature));
//         conversions.push(get_conversionmap_for_lines(temperature2humidity));
//         conversions.push(get_conversionmap_for_lines(humidity2location));
//         conversions
//     }
// }
//
// fn solve_conversion(conversion_map: &Vec<ConversionMap>, number_to_convert: i128) -> i128 {
//     let mut foundconversionvalue  :i128 = 0;
//     let mut foundconversion = false;
//     for conversion  in  conversion_map {
//         if conversion.source <= number_to_convert
//             && (conversion.source + conversion.size) > number_to_convert
//         {
//             foundconversionvalue = conversion.destination + (number_to_convert - conversion.source);
//             foundconversion = true;
//             break;
//         }
//     }
//     // if not found in conversion it is same num
//     if !foundconversion {
//         return number_to_convert
//     }
//     return foundconversionvalue
// }
//
// fn get_conversionmap_for_lines(lines: &[String]) -> Vec<ConversionMap> {
//     let mut conversions: Vec<ConversionMap> = vec![];
//     for line in lines {
//         let split_input: Vec<&str> = line.split_whitespace().collect();
//         let destination_start = num_util::parse_string_ref(split_input[0]);
//         let source_start = num_util::parse_string_ref(split_input[1]);
//         let amount = num_util::parse_string_ref(split_input[2]);
//         conversions.push(ConversionMap {
//             source: source_start,
//             destination: destination_start,
//             size: amount
//         })
//     }
//     conversions
// }
//
// fn process_number(number: i128, conversions: &Vec<Vec<ConversionMap>>) -> i128 {
//     let mut temp_num = number;
//     for map in conversions {
//         temp_num = solve_conversion(&map, temp_num)
//     }
//     return temp_num
// }