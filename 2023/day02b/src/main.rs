use std::cmp::max;

fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let mut sum: u32 = 0;
    for (id, game) in input
        .lines()
        .map(|l| l.split(':').collect::<Vec<_>>())
        .map(|s| (s[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<u16>().unwrap(), s[1].split(';').collect::<Vec<_>>())) {

        let parsed_game = game
            .iter()
            .map(|set| set
                .trim()
                .split(',')
                .map(|a| a.trim().split_whitespace().collect::<Vec<_>>())
                .map(|b| (b[0].parse::<u32>().unwrap(), b[1]))
                .collect::<Vec<(u32, &str)>>()
            )
            .collect::<Vec<Vec<(u32, &str)>>>();

        let mut lowest_number_of_blues_needed: u32 = 0;
        let mut lowest_number_of_reds_needed: u32 = 0;
        let mut lowest_number_of_greens_needed: u32 = 0;
        for set in parsed_game {

            for (cube_count, cube_color) in set {
                if cube_color == "blue" {
                    lowest_number_of_blues_needed = max(lowest_number_of_blues_needed, cube_count)
                }
                else if cube_color == "red" {
                    lowest_number_of_reds_needed = max(lowest_number_of_reds_needed, cube_count)
                }
                else if cube_color == "green" {
                    lowest_number_of_greens_needed = max(lowest_number_of_greens_needed, cube_count)
                }
            }
        }

        sum += lowest_number_of_blues_needed * lowest_number_of_reds_needed * lowest_number_of_greens_needed;
    }

    println!("{sum}")
}
