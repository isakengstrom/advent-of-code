fn main() {

    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let mut stack_numbering_row_index = 0;
    
    let mut create_rows: Vec<&str> = Vec::new();
    
    let mut stack_numbering_row = "";
    
    // Gather the crate stack rows and stack numbering row
    for (row_index, row) in input.lines().enumerate() {
        if !row.contains('[') {
            stack_numbering_row_index = row_index;
            stack_numbering_row = row;
            break;
        }
        
        create_rows.push(row)
    }
    
    // Take the values of the crates. 
    let crate_values: Vec<Vec<char>> = create_rows.iter().rev().map(|&row| row.chars().skip(1).step_by(4).collect()).collect();
    
    
    let num_crate_stacks = stack_numbering_row.trim_end().chars().last().unwrap().to_digit(10).unwrap();
    let mut crate_stacks:Vec<Vec<char>> = vec![Vec::new(); num_crate_stacks as usize];
    
    // Initialize the data structure representing the changeable crate stacks, with the initial crate setup. 
    for crates_of_row in crate_values {
        for (stack_index, value) in crates_of_row.iter().enumerate() {
            if !value.is_ascii_whitespace() {
                crate_stacks[stack_index].push(*value);
            }
        }
    }
    
    let instructions_start_row_index = stack_numbering_row_index + 2;
    
    // Move the crates by the instructions
    for instruction_row in input.lines().skip(instructions_start_row_index) {
        let instructions: Vec<&str> = instruction_row.split_whitespace().collect();
        let (num_to_move, from_stack_num, to_stack_num):(usize, usize, usize) = (instructions[1].parse().unwrap(), instructions[3].parse().unwrap(), instructions[5].parse().unwrap());

        let mut from_stack = crate_stacks[from_stack_num - 1].clone();
        let to_stack = &mut crate_stacks[to_stack_num - 1];
        
        // Move crates from a stack to another, one by one
        for _ in 0..num_to_move {
            to_stack.push(from_stack.pop().unwrap())
        }
        
        crate_stacks[from_stack_num - 1] = from_stack;
    }

    // Crate string with the top crate values after crates have finished moving
    let mut final_top_crates = String::from("");
    for crate_stack in crate_stacks {
        final_top_crates.push(*crate_stack.last().unwrap())
    }
    
    println!("{final_top_crates}")
}
