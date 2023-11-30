fn main() {
    //let mut input = include_str!("../input.txt").to_string();
    let input = include_str!("../input.txt");
    
    let rucksacks = input
        .lines()
        .map(|r| r.split_at(r.len() / 2))
        .collect::<Vec<(&str, &str)>>();

    let ascii_upper_case_location = 65;
    let ascii_lower_case_location = 97;
    
    let mut score: u32 = 0;
    for compartments in rucksacks {
        for c in compartments.0.chars() {
            if !compartments.1.contains(c) {
                continue
            }

            let ascii_value = c as u32; // Item char representation to ascii int

            if ascii_value >= ascii_lower_case_location {
                score += ascii_value - ascii_lower_case_location + 1;
            }
            else {
                score += ascii_value - ascii_upper_case_location + 27;
            }
            
            break;
        }
    }
    
    println!("{}", score);
}
