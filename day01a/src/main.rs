
fn main() {
    let mut input = std::fs::read_to_string("input.txt").expect("Inputs loaded");

    // input may have a disgusting bom char in the beginning. If so, remove it  
    if input.starts_with('\u{feff}') {
        input.remove(0);
    }
    
    // Split the items up by elf (separated by empty lines)
    let elves = input.split("\r\n\r\n").collect::<Vec<&str>>();

    // Mutable value for keeping track of the sum of calories, of the elf carrying the most calories
    let mut highest_amount_of_calories = 0;
    
    for elf in elves {

        let total_calories_of_elf: u32 = elf
            .lines() // Get each item, separated by lines
            .map(|food_item| food_item.trim().parse::<u32>().unwrap()) // parse values from string to int
            .sum(); // sum the calories of all items of the elf

        if total_calories_of_elf > highest_amount_of_calories {
            highest_amount_of_calories = total_calories_of_elf
        }
    }
    
    println!("The elf with most calories has: {highest_amount_of_calories} cals.");
    
}
