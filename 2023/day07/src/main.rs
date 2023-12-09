mod card_hand;

use std::collections::{BinaryHeap, HashMap};
use crate::card_hand::CardHand;

fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let parsed = parse(input);

    let ans_part_1 = part_1(parsed.clone());
    let ans_part_2 = part_2();

    println!("Part 1: {}", {ans_part_1});
    println!("Part 2: {}", {ans_part_2});
}


fn parse(input: &str) -> HashMap<&str, u32> {
    let parsed_lines: Vec<(&str, u32)> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<_>>())
        .map(|l| (l[0], l[1].parse::<u32>().unwrap()))
        .collect();

    let mut map: HashMap<&str, u32> = HashMap::with_capacity(parsed_lines.len());

    for line in parsed_lines {
        map.insert(line.0, line.1);
    }

    return map
}

fn part_1(parsed:HashMap<&str, u32>) -> u64 {

    let mut min_heap: BinaryHeap<CardHand> = BinaryHeap::new();

    for p in &parsed {
        min_heap.push(CardHand::new(p.0.to_string()))
    }

    let mut result: u64 = 0;
    for index in 0..min_heap.len() {
        let hand = min_heap.pop().unwrap().value;
        let bid = parsed.get(hand.as_str()).unwrap();
        result += ((index as u32 + 1) * bid) as u64;
    }

    return result
}


fn part_2() -> u32 {
    return 0
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn part_1_output_should_be_correct() {
        let input = include_str!("../input.example.txt");
        let parsed = parse(input);
        assert_eq!(part_1(parsed.clone()), 6440);
    }

    //#[test]
    //fn test_2_output_should_be_correct() {
    //    let input = include_str!("../input.example.txt");
    //    let parsed = parse(input);
    //    assert_eq!(part_2(parsed), 0)
    //}
}