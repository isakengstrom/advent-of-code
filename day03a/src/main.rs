

fn main() {
    //let mut input = include_str!("../input.txt").to_string();
    let mut input = include_str!("../input.txt").to_string();

    // input may have a disgusting bom char in the beginning. If so, remove it  
    if input.starts_with('\u{feff}') {
        input.remove(0);
    }
    
    let rucksacks = input
        .lines()
        .map(|r| r.split_at(r.len() / 2)).collect::<Vec<(&str, &str)>>();

    
    let mut score: u32 = 0;
    for compartments in rucksacks {
        for c in compartments.0.chars() {
            if compartments.1.contains(c) {
                let ascii_value = c as u32;
                
                if ascii_value >= 97 { 
                    score += ascii_value - 97 + 1;
                }
                else {
                    score += ascii_value - 65 + 27;
                }
                break;
            }
        }
    }
    
    println!("{}", score);
}
