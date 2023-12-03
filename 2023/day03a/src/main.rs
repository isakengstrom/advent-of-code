use std::collections::HashMap;

fn main() {
    //let input = include_str!("../input.example.txt");
    //let input = include_str!("../input.example.7.txt");
    let input = include_str!("../input.txt");

    let mut table: HashMap<String, u32> = HashMap::new();

    for (schematic_row_index, schematic_row) in input.lines().enumerate() {
        for (schematic_col_index, schematic_symbol) in schematic_row.chars().enumerate() {
            if !schematic_symbol.is_ascii_digit() {
                continue
            }

            let schematic_row_chars = schematic_row.chars().collect::<Vec<_>>();
            let mut num: Vec<char> = vec![];
            num.push(schematic_symbol);
            let mut end_index_of_number: usize = 0;

            for potential_number_index in schematic_col_index+1..schematic_row_chars.len() {
                let potential_number = schematic_row_chars[potential_number_index];
                end_index_of_number = potential_number_index;
                if !potential_number.is_ascii_digit() {
                    break
                }
                num.push(potential_number);
            }

            let number = num.into_iter().collect::<String>().parse::<u32>().unwrap();
            let q = schematic_col_index..end_index_of_number;
            for a in q {
                let key = format!("{}:{}",schematic_row_index, a);
                let value = number;
                if table.contains_key(&key.clone()) {
                    break
                    //continue
                }
                table.insert(key.clone(), value);
                //println!("key {key}, value {value}")
            }
        }
    }

    let mut result: u32 = 0;

    //let mut whet: Vec<char> = vec![];
    for (schematic_row_index, schematic_row) in input.lines().enumerate() {
        for (schematic_col_index, schematic_symbol) in schematic_row.chars().enumerate() {
            if schematic_symbol.is_ascii_digit() || schematic_symbol == '.' {
                continue
            }

            //whet.push(schematic_symbol);

            let keys_above: Vec<String> = vec![
                format!("{}:{}", schematic_row_index as i32 -1, schematic_col_index as i32 -1),
                format!("{}:{}", schematic_row_index as i32 -1, schematic_col_index as i32),
                format!("{}:{}", schematic_row_index as i32 -1, schematic_col_index as i32 +1),
            ];


            let mut above_values: Vec<u32> = vec![];
            for above_key in keys_above {
                if above_key.contains('-') {
                    continue
                }
                if table.contains_key(&above_key) {
                    above_values.push(*table.get(&above_key).unwrap())
                }
                else {
                    above_values.push(0);
                }
            }
            above_values.dedup();

            for above_value in above_values {
                result += above_value;
            }

            let keys_below: Vec<String> = vec![
                format!("{}:{}", schematic_row_index as i32 +1, schematic_col_index as i32 -1),
                format!("{}:{}", schematic_row_index as i32 +1, schematic_col_index as i32),
                format!("{}:{}", schematic_row_index as i32 +1, schematic_col_index as i32 +1)
            ];

            let mut below_values: Vec<u32> = vec![];
            for below_key in keys_below {
                if below_key.contains('-') {
                    continue
                }
                if table.contains_key(&below_key) {
                    below_values.push(*table.get(&below_key).unwrap())
                }
                else {
                    below_values.push(0);
                }
            }
            below_values.dedup();

            for below_value in below_values {
                result += below_value;
            }

            let keys_in_row: Vec<String> = vec![
                format!("{}:{}", schematic_row_index as i32, schematic_col_index as i32 -1),
                format!("{}:{}", schematic_row_index as i32, schematic_col_index as i32 +1),
            ];

            for in_row_key in keys_in_row {
                if in_row_key.contains('-') {
                    continue
                }
                if table.contains_key(&in_row_key){
                    result += table.get(&in_row_key).unwrap();
                }
            }
        }
    }
    println!("{result}")
    // 530537
}
