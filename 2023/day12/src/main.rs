
#[derive(Clone, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input.clone()));
}

fn arrange(springs: &Vec<Spring>, damaged_group_sizes: &Vec<usize>) -> u32 {

    // 1. Is valid arrangement if there are no more Damaged groups and no more Damaged springs remaining.
    // All potential remaining Unknown springs are treated as Operational.
    // 2. Is invalid arrangement if there are no more Damaged groups, but there are Damaged springs remaining.
    if damaged_group_sizes.is_empty() {
        let any_remaining_damaged_springs = springs.iter().any(|s| *s == Spring::Damaged);
        return if any_remaining_damaged_springs { 0 } else { 1 };
    }

    // Is invalid arrangement if there are fewer springs left than there must be, according to the remaining damaged groups
    let num_springs_in_between_damaged_groups =  damaged_group_sizes.len() - 1;
    let min_num_springs_according_to_damaged_groups = damaged_group_sizes.iter().sum::<usize>() + num_springs_in_between_damaged_groups;
    if springs.len() < min_num_springs_according_to_damaged_groups {
        return 0;
    }

    let current_spring = springs.first().unwrap();

    // If reached, there are both springs and damaged groups remaining
    // Move on to the next spring if the current one is Operational.
    if *current_spring == Spring::Operational {
        return arrange(&springs[1..].to_vec(), damaged_group_sizes)
    }

    // At this point, current spring is either Damaged or Unknown

    let mut num_arrangements = 0;

    let current_damaged_group_size = *damaged_group_sizes.first().unwrap() as usize;
    let any_operational_in_group = (0..current_damaged_group_size).any(|i| springs[i] == Spring::Operational);
    let group_size_and_break_is_valid = springs.len() > current_damaged_group_size && springs[current_damaged_group_size] != Spring::Damaged;
    let num_total_and_group_size_is_same = springs.len() == current_damaged_group_size;

    // At this point, spring is Damaged or Unknown. Here, treat it as a Damaged spring, as long as following criteria are met:
    // 1. Can only be a valid arrangement if there are no operational springs in the group.
    // 2. Can only be a valid arrangement if the spring after a group is not a Damaged spring.
    if !any_operational_in_group && (group_size_and_break_is_valid || num_total_and_group_size_is_same) {
        if current_damaged_group_size < springs.len() - 1 {
            num_arrangements += arrange(&springs[current_damaged_group_size+1..].to_vec(), &damaged_group_sizes[1..].to_vec())
        }
        else {
            num_arrangements += arrange(&vec![], &damaged_group_sizes[1..].to_vec());
        }
    }

    // If spring is Unknown, treat it as Operational spring and see if it lead to valid arrangement
    if *current_spring == Spring::Unknown {
        num_arrangements += arrange(&springs[1..].to_vec(), damaged_group_sizes)
    }

    num_arrangements
}
fn part_1(input: &str) -> u32 {
    parse(input)
        .iter()
        .map(|(springs, damaged_group_sizes)| arrange(springs, damaged_group_sizes))
        .sum()
}

fn part_2(input: &str) -> u32 {
    0
}



fn parse(input: &str) -> Vec<(Vec<Spring>, Vec<usize>)> {
    let parsed: Vec<(Vec<Spring>, Vec<usize>)>  = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|l| (
            l.0
                .trim_matches('.') // Remove any leading or trailing operational springs
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

    parsed
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
}