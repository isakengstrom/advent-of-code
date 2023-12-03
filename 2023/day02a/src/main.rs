fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let allowed_num_blues:u16 = 14;
    let allowed_num_reds:u16 = 12;
    let allowed_num_greens:u16 = 13;

    let mut matching_id_sum: u16 = 0;

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
                .map(|b| (b[0].parse::<u16>().unwrap(), b[1]))
                .collect::<Vec<(u16, &str)>>()
            )
            .collect::<Vec<Vec<(u16, &str)>>>();

        let mut is_possible_game = true;
        for set in parsed_game {
            for (cube_count, cube_color) in set {
                if cube_color == "blue" && cube_count > allowed_num_blues {
                     is_possible_game = false;
                }
                else if cube_color == "red" && cube_count > allowed_num_reds {
                    is_possible_game = false;
                }
                else if cube_color == "green" && cube_count > allowed_num_greens {
                    is_possible_game = false;
                }
            }
        }

        if is_possible_game {
            matching_id_sum += id;
        }
    }

    println!("{matching_id_sum}")
}
