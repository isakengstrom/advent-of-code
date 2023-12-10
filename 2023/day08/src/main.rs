use std::collections::HashMap;
use num::integer::lcm;
fn main() {
    let input = include_str!("../input.txt");

    let parsed = parse(input);

    let ans_part_1 = part_1(parsed.0.clone(), parsed.1.clone());
    let ans_part_2 = part_2_lcm(parsed.0.clone(), parsed.1.clone());

    println!("Part 1: {}", {ans_part_1});
    println!("Part 2: {}", {ans_part_2});
}


fn parse(input: &str) -> (Vec<u8>, HashMap<&str, (&str, &str)>) {
    let mut lines_iter = input.lines().into_iter();

    let instructions:Vec<u8> = lines_iter
        .next()
        .unwrap()
        .chars()
        .map(|i| if i == 'L' {0} else {1})
        .collect();

    lines_iter.next();

    let nodes: HashMap<&str, (&str, &str)> = lines_iter
        .map(|l| l.split(['=', '(', ',', ')']).collect::<Vec<_>>())
        .map(|l| (l[0].trim_end(), (l[2], l[3].trim_start())))
        .collect();

    return (instructions, nodes)
}

fn part_1(instructions: Vec<u8>, nodes: HashMap<&str, (&str, &str)>) -> u32 {
    let start = "AAA";
    let end = "ZZZ";

    let mut at = start;
    let mut steps: u32 = 0;
    loop {
        //println!("at: {at}");
        if at == end {
            break;
        }

        let node = nodes.get(at).unwrap();

        let direction = instructions[steps.rem_euclid(instructions.len() as u32) as usize];
        if direction == 0 {
            at = node.0
        }
        else {
            at = node.1
        }

        steps += 1;
    }

    steps
}

fn part_2_lcm(instructions: Vec<u8>, nodes: HashMap<&str, (&str, &str)>) -> u64 {
    let start = 'A';
    let end = 'Z';

    let starting_nodes: Vec<&str> = nodes
        .iter()
        .filter(|(&node_start_pos, _)| node_ends_with(node_start_pos,start))
        .map(|(key, _)| *key)
        .collect();

    let mut cycles: Vec<u32> = vec![0; starting_nodes.len()];

    for (index, start_node) in starting_nodes.iter().enumerate() {
        let mut at = *start_node;
        let mut steps: u32 = 0;
        loop {
            //println!("at: {at}");
            if node_ends_with(at,end) {
                break;
            }

            let node = nodes.get(at).unwrap();

            let direction = instructions[steps.rem_euclid(instructions.len() as u32) as usize];
            if direction == 0 {
                at = node.0
            }
            else {
                at = node.1
            }

            steps += 1;
        }

        cycles[index] = steps
    }

    let mut total_steps: u64 = 1;
    for cycle in cycles {
        total_steps = lcm(total_steps, cycle as u64)
    }

    return total_steps
}

fn part_2_brute(instructions: Vec<u8>, nodes: HashMap<&str, (&str, &str)>) -> u32 {
    let start = 'A';
    let end = 'Z';

    let starting_nodes: Vec<&str> = nodes
        .iter()
        .filter(|(&node_start_pos, _)| node_ends_with(node_start_pos,start))
        .map(|(key, _)| *key)
        .collect();

    let mut ats:Vec<String> = starting_nodes
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<_>>()
        .clone();

    let num_ats = ats.len() as u16;
    let mut steps: u32 = 0;
    loop {
        let mut num_at_an_end: u16 = 0;

        ats = ats
            .iter()
            .map(|at| nodes.get(at.as_str()).unwrap())
            .map(|node| if instructions[steps.rem_euclid(instructions.len() as u32) as usize] == 0 {node.0.to_string()} else {node.1.to_string()})
            .collect();

        for at in ats.clone() {
            if node_ends_with(at.as_str(), end) {
                num_at_an_end += 1;
            }
        }

        steps += 1;

        if num_at_an_end == num_ats {
            break;
        }
    }

    steps
}

fn node_ends_with(node: &str, to_end_with: char) -> bool {
    node.chars().last().unwrap() == to_end_with
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_output_should_be_correct_for_example_input_0() {
        let input = include_str!("../input.example.0.txt");
        let parsed = parse(input);
        assert_eq!(part_1(parsed.0, parsed.1), 2)
    }

    #[test]
    fn part_1_output_should_be_correct_for_example_input_1() {
        let input = include_str!("../input.example.1.txt");
        let parsed = parse(input);
        assert_eq!(part_1(parsed.0, parsed.1), 6)
    }

    #[test]
    fn part_2_output_should_be_correct_for_example_input_2() {
        let input = include_str!("../input.example.2.txt");
        let parsed = parse(input);
        assert_eq!(part_2_lcm(parsed.0, parsed.1), 6)
    }
}