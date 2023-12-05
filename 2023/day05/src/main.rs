use std::collections::HashMap;

fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let mut almanac_lines = input.lines();
    let almanac_seeds: Vec<u64> = almanac_lines
        .next()
        .unwrap()
        .split(':')
        .collect::<Vec<_>>()[1]
        .trim()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    almanac_lines.next();

    let mut almanac_maps: HashMap<&str, (&str, Vec<(u64, u64, u64)>)> = HashMap::new();

    let mut source = "";
    let mut destination = "";
    let mut number_instructions: Vec<(u64, u64, u64)> = vec![];
    for line in almanac_lines {
        if line.is_empty() {
            almanac_maps.insert(source, (destination, number_instructions.clone()));
            source = "";
            destination = "";
            number_instructions = vec![];
            continue
        }

        if source == "" {
            let map_instruction_line: Vec<_> = line.split(['-', ' ']).collect();
            source = map_instruction_line[0];
            destination = map_instruction_line[2];
            continue
        }

        let number_instructions_line: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        number_instructions.push((number_instructions_line[0], number_instructions_line[1], number_instructions_line[2]))
    }

    // Add final map
    almanac_maps.insert(source, (destination, number_instructions.clone()));

    let ans_part_1 = part_1(almanac_seeds.clone(), almanac_maps.clone());
    let ans_part_2 = part_2(almanac_seeds.clone(), almanac_maps.clone());
    println!("Part 1: {ans_part_1}");
    println!("Part 2: {ans_part_2}")
}

fn part_1(almanac_seeds: Vec<u64>, almanac_maps: HashMap<&str, (&str, Vec<(u64, u64, u64)>)>) -> u64 {
    let mut key = "seed";

    let mut values = almanac_seeds.clone();
    loop {
        if !almanac_maps.contains_key(key) {
            break
        }

        let map = almanac_maps.get(key).unwrap();
        let instructions = map.clone().1;

        let mut new_values: Vec<u64> = vec![];
        for value in values.clone() {
            let mut continue_outer = false;
            for instruction in &instructions {
                let value_min = instruction.1;
                let value_max = instruction.1 + instruction.2;
                if value < value_min || value >= value_max {
                    continue;
                }
                let add = value - instruction.1;
                let new_value = instruction.0 + add;
                new_values.push(new_value);
                continue_outer = true;
            }
            if continue_outer {
                continue
            }
            new_values.push(value);
        }

        values = new_values;
        key = map.0;
    }

    return *values.iter().min().unwrap();
}

fn part_2(almanac_seeds: Vec<u64>, almanac_maps: HashMap<&str, (&str, Vec<(u64, u64, u64)>)>) -> u64 {
    return 0;
}