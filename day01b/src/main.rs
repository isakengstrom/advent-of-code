fn main() {
    let mut input = std::fs::read_to_string("input.txt").expect("Inputs loaded");

    // input may have a disgusting bom char in the beginning. If so, remove it  
    if input.starts_with('\u{feff}') {
        input.remove(0);
    }
    
    // Split the items up by elf (separated by empty lines)
    let elves = input.split("\r\n\r\n").collect::<Vec<&str>>();
    
    let mut top_three: Vec<u32> = vec![0; 3];
    let mut index_of_elf_with_fewest_cals = 0;
    
    for elf in elves {

        let total_calories_of_elf: u32 = elf
            .lines() // Get each item, separated by lines
            .map(|food_item| food_item.trim().parse::<u32>().unwrap()) // parse values from string to int
            .sum(); // sum the calories of all items of the elf
        
        if total_calories_of_elf > top_three[index_of_elf_with_fewest_cals] {
            top_three[index_of_elf_with_fewest_cals] = total_calories_of_elf;
            index_of_elf_with_fewest_cals = get_index_of_lowest(top_three.clone());
        }
    }
    
    let total: u32 = top_three.iter().sum();
    
    println!("The top three elves have {total} cals in total.");
    
}


fn get_index_of_lowest(vals: Vec<u32>) -> usize {

    let mut min = vals[0];
    let mut min_index = 0;

    for (index, &val) in vals.iter().enumerate() {
        if val < min {
            min = val;
            min_index = index;
        }
    }
    
    return min_index;
}
