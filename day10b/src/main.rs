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

    let mut crt_line: Vec<char> = vec![];
    for (cycle, circuit_value) in circuits.iter().enumerate() {

        if ((cycle as i32) % 40 - signal_register).abs() < 2 {
            crt_line.push('#');
        }
        else {
            crt_line.push('.');
        }
        
        if cycle == 39 || cycle == 79 || cycle == 119 || cycle == 159 || cycle == 199 || cycle == 239 {
            let line: String = crt_line.iter().collect();
            crt_line.clear();
            print!("{line}\n")
        }

        signal_register += circuit_value;
    }
}