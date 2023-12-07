use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use crate::{num_util, solver};
use crate::day7::KindOfHand::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

pub struct SolverImpl;

pub const NUMBER_AND_VALUES: &[(&str, i32); 13] = &[
    ("A",14),
    ("K",13),
    ("Q",12),
    ("J",11),
    ("T",10),
    ("9",9),
    ("8",8),
    ("7",7),
    ("6",6),
    ("5",5),
    ("4",4),
    ("3",3),
    ("2",2)];

enum KindOfHand {
    FiveOfAKind = 600,
    FourOfAKind = 500,
    FullHouse = 400,
    ThreeOfAKind = 300,
    TwoPair = 200,
    OnePair = 100,
    HighCard = 0
}

#[derive(Debug)]
struct Card {
    value: String,
}

impl Eq for Card {}

impl Card {
    fn custom_comp(&self, card: &Card) -> Ordering {
        let value1 = self.card_value();
        let value2 = card.card_value();
        value1.cmp(&value2)
    }
    fn card_value(&self) -> i32 {
        let cardvalue = match NUMBER_AND_VALUES.iter().find(|&&c| c.0 == self.value.to_string()) {
            Some(&(_, card_value)) => card_value,
            None => 0,
        };
        cardvalue
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Hash for Card {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.value.hash(hasher);
    }
}


impl solver::Solver for SolverImpl {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128 {
        let mut res = 0;

        let mut hand_and_bid: Vec<(Vec<Card>, i128)> = vec![];
        for line in inputs {
            let hand = line.split_whitespace().collect::<Vec<_>>()[0];
            let bid = num_util::parse_string_ref(line.split_whitespace().collect::<Vec<_>>()[1]);
            let handcards: Vec<Card> = hand.chars().map(|d| Card { value: d.to_string() }).collect();
            hand_and_bid.push((handcards, bid))
        }

        let mut hand_with_value: Vec<((Vec<Card>, i128), i32)> = vec![];
        for h in hand_and_bid {
            let value = get_hand_value(&h.0);
            hand_with_value.push((h, value));
        }

        hand_with_value.sort_by(|a, b| {
            let value_cmp = a.1.cmp(&b.1);
            if value_cmp == Ordering::Equal {
                compare_card_vectors(&a.0.0, &b.0.0)
            } else {
                value_cmp
            }
        });

        let mut count: i128 = 1;
        for hand in hand_with_value {
            res += hand.0.1 * count ;
            count += 1;
        }

        return res
    }

    fn solve_part2(&self, _inputs: &Vec<String>) -> i128 {
        let res = 0;
        return res
    }
}

fn get_hand_value(hand: &Vec<Card>) -> i32 {
    let kind_of_hand = check_kinds_parts(&hand);
    return kind_of_hand as i32
}

fn check_kinds_parts(hand: &Vec<Card>) -> KindOfHand {
    let mut counts = HashMap::new();

    for (_, element) in hand.iter().enumerate() {
        let count = counts.entry(element).or_insert(0);
        *count += 1;
    }

    let mut count_counts: HashMap<i32, i32> = HashMap::new();
    for (_card, c) in counts{
        let entry = count_counts.entry(c).or_insert(0);
        *entry += 1;
    }

    if count_counts.get(&5).is_some_and(|five| five == &1) { return FiveOfAKind}
    if count_counts.get(&4).is_some_and(|four| four == &1) { return FourOfAKind}
    if count_counts.get(&3).is_some_and(|three| three == &1){
        if count_counts.get(&2).is_some_and(|two| two == &1){
            return FullHouse
        }
        return ThreeOfAKind
    }
    if count_counts.get(&2).is_some_and(|two| two == &2) {
        return TwoPair
    }
    if count_counts.get(&2).is_some_and(|two| two == &1) {
        return OnePair
    }
    else {
        return HighCard
    }
}

fn compare_card_vectors(cards1: &[Card], cards2: &[Card]) -> Ordering {
    for (card1, card2) in cards1.iter().zip(cards2) {
        let cmp = card1.custom_comp(card2);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    Ordering::Equal
}