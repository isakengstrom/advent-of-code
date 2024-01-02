mod maze;
mod position;
mod tile;

use std::collections::HashSet;
use crate::maze::Maze;

fn main() {
    let input = include_str!("../input.txt");
    let parsed = parse(input);

    let ans_part_1 = part_1(parsed.clone());
    let ans_part_2 = part_2(parsed.clone());

    println!("Part 1: {}", {ans_part_1});
    println!("Part 2: {}", {ans_part_2});
}

fn parse(input: &str) -> Maze {
    let maze = Maze::new(input);
    maze
}

fn part_1(maze: Maze) -> u32 {
    let start_tile = maze.start_tile;
    let max_steps_around = (maze.width*maze.height) as u32;
    let mut steps_around:u32 = 1;
    let mut current = maze.get_tile(start_tile.connection_2);
    let mut visited_tiles = HashSet::from([current.position]);

    loop {
        if steps_around > max_steps_around {
            panic!("Took more steps than there are tiles, something has gone terribly wrong")
        }

        if steps_around > 3 && current.is_start {
            break
        }

        current = if visited_tiles.contains(&current.connection_1) ||
            (steps_around < 3 && current.connection_1 == start_tile.position){
            maze.get_tile(current.connection_2)
        }
        else {
            maze.get_tile(current.connection_1)
        };

        visited_tiles.insert(current.position);

        steps_around += 1;
    }

    return steps_around / 2
}

fn part_2(parsed: Maze) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_output_should_be_correct_for_example_input_p2_0()
    {
        let input = include_str!("../input.example.p2.0.txt");
        let parsed = parse(input);
        assert_eq!(part_2(parsed), 4);
    }

    #[test]
    fn part_2_output_should_be_correct_for_example_input_p2_1()
    {
        let input = include_str!("../input.example.p2.1.txt");
        let parsed = parse(input);
        assert_eq!(part_2(parsed), 4);
    }

    #[test]
    fn part_2_output_should_be_correct_for_example_input_p2_2()
    {
        let input = include_str!("../input.example.p2.2.txt");
        let parsed = parse(input);
        assert_eq!(part_2(parsed), 8);
    }

    #[test]
    fn part_2_output_should_be_correct_for_example_input_p2_3()
    {
        let input = include_str!("../input.example.p2.3.txt");
        let parsed = parse(input);
        assert_eq!(part_2(parsed), 10);
    }

    #[test]
    fn part_1_output_should_be_correct_for_example_input_0()
    {
        let input = include_str!("../input.example.0.txt");
        let parsed = parse(input);
        assert_eq!(part_1(parsed), 4);
    }

    #[test]
    fn part_1_output_should_be_correct_for_example_input_1()
    {
        let input = include_str!("../input.example.1.txt");
        let parsed = parse(input);
        assert_eq!(part_1(parsed), 4);
    }

    #[test]
    fn part_1_output_should_be_correct_for_example_input_2()
    {
        let input = include_str!("../input.example.2.txt");
        let parsed = parse(input);
        assert_eq!(part_1(parsed), 8);
    }

    #[test]
    fn part_1_output_should_be_correct_for_example_input_3()
    {
        let input = include_str!("../input.example.3.txt");
        let parsed = parse(input);
        assert_eq!(part_1(parsed), 8);
    }
}