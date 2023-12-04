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

    let num_cards = cards.len() as u32;

    let mut copies = vec![0; cards.len()];
    for card in cards {
        let card_number = card.0;
        let winning_numbers: HashSet<u32> = HashSet::from_iter(card.1[0].iter().cloned());
        let numbers_you_have = card.1[1].clone();

        let mut num_winning_numbers = 0;
        for number in numbers_you_have {
            if winning_numbers.contains(&number) {
                num_winning_numbers += 1;
            }
        }

        let num_extra_copies = copies[(card_number-1) as usize];
        let range = card_number-1 .. card_number-1+num_winning_numbers;
        for index in range {
            if index as usize > copies.len() {
                continue
            }
            copies[index as usize + 1] +=  1 + num_extra_copies;
        }
    }

    let total_num_copies = copies.iter().sum::<u32>();
    let total_num_cards = total_num_copies + num_cards;

    println!("{total_num_cards}")
}
