use std::collections::HashMap;

fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let spelled_out_to_digit_map: HashMap<String, char> = HashMap::from([
        ("one".parse().unwrap(), '1'),
        ("two".parse().unwrap(), '2'),
        ("three".parse().unwrap(), '3'),
        ("four".parse().unwrap(), '4'),
        ("five".parse().unwrap(), '5'),
        ("six".parse().unwrap(), '6'),
        ("seven".parse().unwrap(), '7'),
        ("eight".parse().unwrap(), '8'),
        ("nine".parse().unwrap(), '9'),
    ]);

    let mut corrected_input: Vec<char> = vec![];
    for (index, current_char) in input.chars().enumerate() {

        let three_char_substring: String = input.chars().skip(index).take(3).collect();
        let three_match_found = spelled_out_to_digit_map.contains_key(&three_char_substring);
        if three_match_found {
            corrected_input.push(spelled_out_to_digit_map.get(&three_char_substring).unwrap().to_ascii_lowercase());
            continue
        }

        let four_char_substring: String = input.chars().skip(index).take(4).collect();
        let four_match_found = spelled_out_to_digit_map.contains_key(&four_char_substring);
        if four_match_found {
            corrected_input.push(spelled_out_to_digit_map.get(&four_char_substring).unwrap().to_ascii_lowercase());
            continue
        }

        let five_char_substring: String = input.chars().skip(index).take(5).collect();
        let five_match_found = spelled_out_to_digit_map.contains_key(&five_char_substring);
        if five_match_found {
            corrected_input.push(spelled_out_to_digit_map.get(&five_char_substring).unwrap().to_ascii_lowercase());
            continue
        }

        corrected_input.push(current_char);
    }

    let sum = corrected_input
        .into_iter()
        .collect::<String>()
        .as_str()
        .lines()
        .map(|l| (l.chars().find(|&x| x.is_digit(10)).unwrap(), l.chars().rfind(|&x| x.is_digit(10)).unwrap()))
        .map(|t| format!("{}{}", t.0, t.1).parse::<u32>().unwrap())
        .sum::<u32>();

    print!("{sum}");
}
