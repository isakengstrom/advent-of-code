use std::collections::HashSet;

fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let cards = input
        .lines()
        .map(|l| l.split(':').collect::<Vec<_>>())
        .map(|l| (
            l[0].split_ascii_whitespace().collect::<Vec<_>>()[1].parse::<u32>().unwrap(),
            l[1].split('|')
                .collect::<Vec<_>>()
                .iter()
                .map(|nums| nums.trim().split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>())
                .collect::<Vec<Vec<u32>>>()
        ))
        .collect::<Vec<(u32, Vec<Vec<u32>>)>>();

    let mut total_points = 0;
    for card in cards {
        let mut num_winning_numbers = 0;
        let winning_numbers: HashSet<u32> = HashSet::from_iter(card.1[0].iter().cloned());
        let numbers_you_have = card.1[1].clone();

        for number in numbers_you_have {
            if winning_numbers.contains(&number) {
                num_winning_numbers += 1;
            }
        }

        if num_winning_numbers == 0 {
            continue
        }
        let mut points_of_card = 1;
        for _ in 1..num_winning_numbers {
            points_of_card = points_of_card * 2;
        }
        total_points += points_of_card;
    }

    println!("{total_points}")
}
