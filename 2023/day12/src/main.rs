use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn arrange(springs: &Vec<Spring>, damaged_group_sizes: &Vec<usize>, cache: &mut HashMap<(Vec<Spring>,Vec<usize>), u64>) -> u64 {
    // Return cached result if there's a cache hit
    if let Some(&cached_result) = cache.get(&(springs.clone(), damaged_group_sizes.clone())) {
        return cached_result
    }

    // 1. Is valid arrangement if there are no more Damaged groups and no more Damaged springs remaining.
    // All potential remaining Unknown springs are treated as Operational.
    // 2. Is invalid arrangement if there are no more Damaged groups, but there are Damaged springs remaining.
    if damaged_group_sizes.is_empty() {
        let any_remaining_damaged_springs = springs.iter().any(|s| *s == Spring::Damaged);
        if any_remaining_damaged_springs {
            cache.insert((springs.clone(), damaged_group_sizes.clone()), 0);
            return 0
        }
        else {
            cache.insert((springs.clone(), damaged_group_sizes.clone()), 1);
            return 1
        }
    }

    // Is invalid arrangement if there are fewer springs left than there must be, according to the remaining damaged groups
    let num_springs_in_between_damaged_groups =  damaged_group_sizes.len() - 1;
    let min_num_springs_according_to_damaged_groups = damaged_group_sizes.iter().sum::<usize>() + num_springs_in_between_damaged_groups;
    if springs.len() < min_num_springs_according_to_damaged_groups {
        cache.insert((springs.clone(), damaged_group_sizes.clone()), 0);
        return 0;
    }

    let current_spring = springs.first().unwrap();

    // If reached, there are both springs and damaged groups remaining
    // Move on to the next spring if the current one is Operational.
    if *current_spring == Spring::Operational {
        let num_arrangements = arrange(&springs[1..].to_vec(), damaged_group_sizes, cache);
        cache.insert((springs.clone(), damaged_group_sizes.clone()), num_arrangements);
        return num_arrangements
    }

    // At this point, current spring is either Damaged or Unknown

    let mut num_arrangements = 0;

    let current_damaged_group_size = *damaged_group_sizes.first().unwrap();
    let any_operational_in_group = (0..current_damaged_group_size).any(|i| springs[i] == Spring::Operational);
    let group_size_and_break_is_valid = springs.len() > current_damaged_group_size && springs[current_damaged_group_size] != Spring::Damaged;
    let num_total_and_group_size_is_same = springs.len() == current_damaged_group_size;

    // At this point, spring is Damaged or Unknown. Here, treat it as a Damaged spring, as long as following criteria are met:
    // 1. Can only be a valid arrangement if there are no operational springs in the group.
    // 2. Can only be a valid arrangement if the spring after a group is not a Damaged spring.
    if !any_operational_in_group && (group_size_and_break_is_valid || num_total_and_group_size_is_same) {
        if current_damaged_group_size < springs.len() - 1 {
            num_arrangements += arrange(&springs[current_damaged_group_size+1..].to_vec(), &damaged_group_sizes[1..].to_vec(), cache)
        }
        else {
            num_arrangements += arrange(&vec![], &damaged_group_sizes[1..].to_vec(), cache);
        }
    }

    // If spring is Unknown, also treat it as Operational spring and see if it lead to valid arrangement
    if *current_spring == Spring::Unknown {
        num_arrangements += arrange(&springs[1..].to_vec(), damaged_group_sizes, cache)
    }

    cache.insert((springs.clone(), damaged_group_sizes.clone()), num_arrangements);
    num_arrangements
}


fn parse(input: &str) -> Vec<(Vec<Spring>, Vec<usize>)> {
    let records: Vec<(Vec<Spring>, Vec<usize>)>  = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|l| (
            l.0
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => panic!("encountered an unknown spring identifier")
                })
                .collect(),
            l.1
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect())
        )
        .collect();

    records
}

fn expand(records: Vec<(Vec<Spring>, Vec<usize>)>) -> Vec<(Vec<Spring>, Vec<usize>)> {
    let num_copies: usize = 5;
    let num_delimiters = num_copies - 1;

    records
        .iter()
        .map(|record| (
            record.0
                .iter()
                .cloned()
                .chain(vec![Spring::Unknown])
                .cycle()
                .take(record.0.len()*num_copies+num_delimiters)
                .collect(),
            record.1
                .iter()
                .cloned()
                .cycle()
                .take(record.1.len()*num_copies)
                .collect()
            )
        )
        .collect::<Vec<(Vec<Spring>, Vec<usize>)>>()
}

fn part_1(input: &str) -> u64 {
    // Init memoization cache
    let mut cache= HashMap::new();

    parse(input)
        .iter()
        .map(|(springs, damaged_group_sizes)| arrange(springs, damaged_group_sizes, &mut cache))
        .sum()
}

fn part_2(input: &str) -> u64 {
    // Init memoization cache
    let mut cache = HashMap::new();

    let records = parse(input);
    let expanded_records = expand(records);
    expanded_records
        .iter()
        .map(|(springs, damaged_group_sizes)| arrange(springs, damaged_group_sizes, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_output_should_be_correct()
    {
        // Arrange
        let input = include_str!("../input.example.0.txt");

        // Act
        let result = part_1(input);

        // Assert
        assert_eq!(result, 21)
    }

    #[test]
    fn part_2_output_should_be_correct()
    {
        // Arrange
        let input = include_str!("../input.example.0.txt");

        // Act
        let result = part_2(input);

        // Assert
        assert_eq!(result, 525_152)
    }
}