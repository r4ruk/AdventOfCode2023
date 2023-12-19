use std::collections::HashMap;
use crate::file_reader::read_file_to_str;
use crate::solver;
extern crate aoc_lib;


#[derive(Clone, Copy)]
enum RuleKind {
    LessThan,
    GreaterThan,
    Default,
}

#[derive(Clone)]
struct Rule {
    kind: RuleKind,
    subject: usize,
    benchmark: usize,
    destination: String,
}

struct Workflow {
    rules: Vec<Rule>,
}

struct Rating {
    value: [usize; 4],
    total: usize,
}

impl Rating {
    fn is_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut curr_workflow = workflows.get("in").unwrap();
        loop {
            let mut destination: String = "R".to_string();
            for rule in &curr_workflow.rules {
                match rule.kind {
                    RuleKind::LessThan => {
                        if self.value[rule.subject] < rule.benchmark {
                            destination = rule.destination.clone();
                            break;
                        }
                    }
                    RuleKind::GreaterThan => {
                        if self.value[rule.subject] > rule.benchmark {
                            destination = rule.destination.clone();
                            break;
                        }
                    }
                    RuleKind::Default => {
                        destination = rule.destination.clone();
                    }
                }
            }
            match destination.chars().next().unwrap() {
                'A' => return true,
                'R' => return false,
                _ => {
                    curr_workflow = workflows.get(&destination).unwrap();
                }
            }
        }
    }
}


pub struct SolverImpl;
impl solver::Solver for SolverImpl {
    fn solve_part1(&self, _inputs: &Vec<String>) -> i128 {
        //name of the rule, then tuple of name of variable used in the rule,then Rule
        let input = &*read_file_to_str(19);
        let binding = input.to_owned().clone();
        // fcking windows (or txt file ?!) uses \r\n for a single new line
        let lines: Vec<_> = binding.split("\r\n\r\n").collect();
        let workflows: HashMap<String, Workflow> = lines[0]
            .lines()
            .map(|line| {
                let line: Vec<_> = line.split('{').collect();
                let name = line[0].trim().to_string();
                let rules: Vec<Rule> = line[1][0..line[1].len() - 1]
                    .split(',')
                    .map(|rule| match rule.find(':') {
                        Some(_) => {
                            let rule: Vec<_> = rule.split(':').collect();
                            let destination: String = rule[1].trim().to_string();
                            let kind: RuleKind = match rule[0].chars().nth(1).unwrap() {
                                '<' => RuleKind::LessThan,
                                '>' => RuleKind::GreaterThan,
                                _ => panic!("Invalid rule {}", rule[0]),
                            };
                            let subject: usize = match rule[0].chars().next().unwrap() {
                                'x' => 0,
                                'm' => 1,
                                'a' => 2,
                                's' => 3,
                                _ => panic!("Invalid rule {}", rule[0]),
                            };
                            let benchmark: usize = rule[0][2..].parse().unwrap();
                            Rule {
                                kind,
                                subject,
                                benchmark,
                                destination,
                            }
                        }
                        None => Rule {
                            kind: RuleKind::Default,
                            subject: 0,
                            benchmark: 0,
                            destination: rule.to_string(),
                        },
                    })
                    .collect();
                (name.to_string(), Workflow { rules })
            })
            .collect();

        let ratings: Vec<Rating> = lines[1]
            .lines()
            .map(|line| {
                let line = line[1..line.len() - 1].split(',').collect::<Vec<_>>();
                let x = line[0][2..].parse().unwrap();
                let m = line[1][2..].parse().unwrap();
                let a = line[2][2..].parse().unwrap();
                let s = line[3][2..].parse().unwrap();
                let value = [x, m, a, s];
                let total = x + m + a + s;
                Rating { value, total }
            })
            .collect();

        let res = ratings
            .iter()
            .filter_map(|rating| {
                if rating.is_accepted(&workflows) {
                    Some(rating.total)
                } else {
                    None
                }
            })
            .sum::<usize>();
        return res as i128;
    }

    fn solve_part2(&self, _inputs: &Vec<String>) -> i128 {
       return 0
    }
}