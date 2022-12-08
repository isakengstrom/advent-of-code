use std::collections::HashSet;

fn main() {
    let ascii_upper_case_location = 65;
    let ascii_lower_case_location = 97;

    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let rucksacks = input.lines().map(|c| c.chars().collect::<HashSet<char>>()).collect::<Vec<HashSet<char>>>();
    
    let mut score: u32 = 0;
    for rucksack_trio in rucksacks.chunks(3) {
        let uniques_in_first_two = rucksack_trio[0].intersection(&rucksack_trio[1]).copied().collect::<HashSet<_>>();
        let ascii_value = uniques_in_first_two.intersection(&rucksack_trio[2]).next().unwrap().clone() as u32;
 
        if ascii_value >= ascii_lower_case_location {
            score += ascii_value - ascii_lower_case_location + 1;
            continue
        }
        
        score += ascii_value - ascii_upper_case_location + 27;
    }

    println!("{}", score);
}
