mod card_hand;
mod card_hand_2;

use std::collections::{BinaryHeap, HashMap};
use crate::card_hand::CardHand;
use crate::card_hand_2::CardHand2;

fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let parsed = parse(input);

    let ans_part_1 = part_1(parsed.clone());
    let ans_part_2 = part_2(parsed.clone());

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

fn part_1(parsed: HashMap<&str, u32>) -> u64 {

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


fn part_2(parsed: HashMap<&str, u32>) -> u64 {
    let mut min_heap: BinaryHeap<CardHand2> = BinaryHeap::new();

    for p in &parsed {
        min_heap.push(CardHand2::new(p.0.to_string()))
    }

    let mut result: u64 = 0;
    for index in 0..min_heap.len() {
        let hand = min_heap.pop().unwrap().value;
        let bid = parsed.get(hand.as_str()).unwrap();
        result += ((index as u32 + 1) * bid) as u64;
        //println!("hand {index}: {hand} - {bid}")
    }

    return result
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

    #[test]
    fn part_1_output_should_be_correct_extra_input_1() {
        let input = include_str!("../input.example.1.txt");
        let parsed = parse(input);
        assert_eq!(part_1(parsed.clone()), 6592);
    }

    /*
        Correct order:
        - 32T3K
        - KK677
        - T55J5
        - QQQJA
        - KTJJT
    */
    #[test]
    fn test_2_output_should_be_correct() {
        let input = include_str!("../input.example.txt");
        let parsed = parse(input);
        assert_eq!(part_2(parsed), 5905)
    }

    /*
       correct order:
       - 2345A 1
       - J345A 2
       - 2345J 3
       - 32T3K 5
       - KK677 7
       - T3Q33 11
       - Q2KJJ 13
       - T3T3J 17
       - Q2Q2Q 19
       - 2AAAA 23
       - T55J5 29
       - QQQJA 31
       - KTJJT 34
       - JJJJJ 37
       - JJJJ2 41
       - JAAAA 43
       - 2JJJJ 53
       - AAAAJ 59
       - AAAAA 61
    */
    #[test]
    fn test_2_output_should_be_correct_extra_input_1() {
        let input = include_str!("../input.example.1.txt");
        let parsed = parse(input);
        assert_eq!(part_2(parsed), 6839)
    }


    /*
        Correct order:
        - KK677 7
        - T3Q33 11
        - Q2KJJ 13
        - T3T3J 17
        - Q2Q2Q 19
        - 2AAAA 23
     */
    #[test]
    fn test_2_output_should_be_correct_extra_input_2() {
        let input = include_str!("../input.example.2.txt");
        let parsed = parse(input);
        assert_eq!(part_2(parsed), 369)
    }
}