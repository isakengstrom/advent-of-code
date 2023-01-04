fn main() {

    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    
    let mut circuits: Vec<i32> = vec![]; 
    
    for instruction_line in input.lines() {
        if instruction_line.starts_with('n') {
            circuits.push(0);
        }
        else {
            circuits.push(0);
            circuits.push(instruction_line.split_once(' ').unwrap().1.parse::<i32>().unwrap());
        }
    }

    let mut signal_register: i32 = 1;
    
    let mut interesting_signal_strength_sum = 0;
    
    for (cycle, circuit_value) in circuits.iter().enumerate() {
        if cycle == 19 || cycle == 59 || cycle == 99 || cycle == 139 || cycle == 179 || cycle == 219 {
            let signal_strength = signal_register * (cycle as i32 + 1);
            interesting_signal_strength_sum += signal_strength;
        }  
        
        signal_register += circuit_value;
    }
    
    print!("{}", interesting_signal_strength_sum)

}