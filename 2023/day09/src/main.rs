
fn main() {
    let input = include_str!("../input.txt");
    let parsed = parse(input);

    let ans_part_1 = part_1(parsed.clone());
    let ans_part_2 = part_2(parsed.clone());

    println!("Part 1: {}", {ans_part_1});
    println!("Part 2: {}", {ans_part_2});
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect()
        )
        .collect::<Vec<Vec<i32>>>()
}

fn part_1(parsed: Vec<Vec<i32>>) -> i32 {
    let mut sum_of_extrapolated_values = 0;

    for history in parsed {
        let mut seqs: Vec<Vec<i32>> = Vec::from([history]);

        loop {
            let current = seqs.last().unwrap();

            if current.len() == 1 {
                break
            }
            if current.iter().all(|v| *v == 0) {
                break
            }

            let next_row: Vec<_> = current
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect();

            seqs.push(next_row)
        }

        let rev_seqs:Vec<Vec<i32>> = seqs
            .clone()
            .iter()
            .rev()
            .map(|v| v.clone())
            .collect();

        let mut placeholder = 0;
        for (index, _seq) in rev_seqs.iter().enumerate() {
            if index == seqs.len() - 1 {
                break
            }

            let last_value_of_next_seq = rev_seqs[index+1].last().unwrap();
            placeholder = last_value_of_next_seq + placeholder;
        }

        sum_of_extrapolated_values += placeholder;
    }

    sum_of_extrapolated_values
}

fn part_2(parsed: Vec<Vec<i32>>) -> i32 {
    let mut sum_of_extrapolated_values = 0;

    for history in parsed {

        let rev_history = history
            .iter()
            .rev()
            .map(|v| v.clone())
            .collect();
        let mut seqs: Vec<Vec<i32>> = Vec::from([rev_history]);

        loop {
            let current = seqs.last().unwrap();

            if current.len() == 1 {
                break
            }
            if current.iter().all(|v| *v == 0) {
                break
            }

            let next_row: Vec<_> = current
                .windows(2)
                .map(|w| w[0] - w[1])
                .collect();

            seqs.push(next_row)
        }

        let rev_seqs:Vec<Vec<i32>> = seqs
            .clone()
            .iter()
            .rev()
            .map(|v| v.clone())
            .collect();

        let mut placeholder = 0;
        for (index, _seq) in rev_seqs.iter().enumerate() {
            if index == seqs.len() - 1 {
                break
            }

            let last_value_of_next_seq = rev_seqs[index+1].last().unwrap();
            placeholder = last_value_of_next_seq - placeholder;
        }

        sum_of_extrapolated_values += placeholder;
    }

    sum_of_extrapolated_values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_output_should_be_correct_for_example_input_0()
    {
        let input = include_str!("../input.example.0.txt");
        let parsed = parse(input);
        assert_eq!(part_1(parsed), 114);
    }

    #[test]
    fn part_2_output_should_be_correct_for_example_input_0()
    {
        let input = include_str!("../input.example.0.txt");
        let parsed = parse(input);
        assert_eq!(part_2(parsed), 2);
    }
}