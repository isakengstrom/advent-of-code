

fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let boat_races = parse(input);

    let ans_part_1 = part_1(boat_races.clone());
    let ans_part_2 = part_2(boat_races.clone());

    println!("Part 1: {}", {ans_part_1});
    println!("Part 2: {}", {ans_part_2});
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let mut lines_iter = input.lines().into_iter();

    let times: Vec<u32> = lines_iter
        .next()
        .unwrap()
        .split(':')
        .collect::<Vec<_>>()[1]
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u32> = lines_iter
        .next()
        .unwrap()
        .split(':')
        .collect::<Vec<_>>()[1]
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let parsed: Vec<_> = (0..times.len())
        .into_iter()
        .map(|i| (times[i], distances[i]))
        .collect();

    return parsed
}

fn part_1(boat_races: Vec<(u32, u32)>) -> u32 {
    let mut num_ways_to_win_per_race: Vec<u32> = vec![];
    for boat_race in boat_races {
        let mut num_ways_to_win: u32 = 0;
        let race_duration = boat_race.0;
        let distance_to_beat = boat_race.1;
        for charge_time in 0..race_duration {
            let speed = charge_time;
            let move_time = race_duration - charge_time;
            let race_distance = speed * move_time;
            if race_distance > distance_to_beat {
                num_ways_to_win += 1;
            }
        }

        num_ways_to_win_per_race.push(num_ways_to_win)
    }

    return num_ways_to_win_per_race
        .iter()
        .product()
}


fn part_2(boat_races: Vec<(u32, u32)>) -> u64 {
    //let mut big_boat_race: (u64, u64);
    let mut big_time_string: String = "".to_owned();
    let mut big_distance_string: String = "".to_owned();
    for boat_race in boat_races {
        big_time_string.push_str(boat_race.0.to_string().as_str());
        big_distance_string.push_str(boat_race.1.to_string().as_str());
    }

    let race_duration = big_time_string.parse::<u64>()        .unwrap();
    let distance_to_beat = big_distance_string.parse::<u64>().unwrap();


    let mut num_ways_to_win: u64 = 0;
    for charge_time in 0..race_duration {
        let speed = charge_time;
        let move_time = race_duration - charge_time;
        let race_distance = speed * move_time;
        if race_distance > distance_to_beat {
            num_ways_to_win += 1;
        }
    }


    return num_ways_to_win;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn part_1_output_should_be_correct() {
        let input = include_str!("../input.example.txt");
        let boat_races = parse(input);
        assert_eq!(part_1(boat_races), 288);
    }

    #[test]
    fn test_2_output_should_be_correct() {
        let input = include_str!("../input.example.txt");
        let boat_races = parse(input);
        assert_eq!(part_2(boat_races), 71503)
    }
}