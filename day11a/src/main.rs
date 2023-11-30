use std::collections::VecDeque;
fn main() {
    
    let input = include_str!("../input.example.txt");
    //let input = include_str!("../input.txt");
    
    let turns = input.split("\r\n\r\n").collect::<Vec<&str>>();
    
    let mut monkeys: Vec<Monkey> = turns
        .iter()
        .map(|&turn| turn.lines().collect::<Vec<&str>>())
        .map(|t| 
            Monkey::new(
                construct_starting_items_from_str(t[1]),
                construct_operation_from_str(t[2]),
                construct_monkey_test_from_str_vec(vec![t[3], t[4], t[5]])
            )
        )
        .collect();

    let mut monkey_inspection_rates: Vec<u32> = vec![0; monkeys.len()];

    for _ in 0..20 {

        let mut monkeys_next: Vec<Monkey> = turns
            .iter()
            .map(|&turn| turn.lines().collect::<Vec<&str>>())
            .map(|t|
                Monkey::new(
                    VecDeque::new(),
                    construct_operation_from_str(t[2]),
                    construct_monkey_test_from_str_vec(vec![t[3], t[4], t[5]])
                )
            )
            .collect();
        
        for (monkey_index, monkey) in monkeys.iter_mut().enumerate() {
            for item in monkey.starting_items.clone() {
                let worry_level: u32 = (monkey.operation)(item).into();
                let relieved_worry_level = worry_level / 3;
                let to_monkey: usize = (monkey.test)(relieved_worry_level).into();
                monkey.dequeue();
                
                monkeys_next[to_monkey].enqueue(relieved_worry_level)
            }
        }
        

        
        //let mut monkeys_next: Vec<Monkey> = turns
        //    .iter()
        //    .map(|&turn| turn.lines().collect::<Vec<&str>>())
        //    .map(|t|
        //        Monkey::new(
        //            VecDeque::new(),
        //            construct_operation_from_str(t[2]),
        //            construct_monkey_test_from_str_vec(vec![t[3], t[4], t[5]])
        //        )
        //    )
        //    .collect();
        //
        //
        //for monkey in monkeys.iter() {
        //    for item in monkey.starting_items.clone() {
        //        let worry_level: u32 = (monkey.operation)(item).into();
        //        let relieved_worry_level = worry_level / 3;
        //        let to_monkey: usize = (monkey.test)(relieved_worry_level).into();
        //        monkeys_next[to_monkey].starting_items.push_back(relieved_worry_level);
        //        monkey_inspection_rates[to_monkey] += 1;
        //    }
        //}
        let i= 0;
    }
    monkey_inspection_rates.sort();
    
    println!("{}", monkey_inspection_rates[monkey_inspection_rates.len()-1]*monkey_inspection_rates[monkey_inspection_rates.len()-2]);
    
    let i = 0;
    
}

fn construct_starting_items_from_str(si: &str) -> VecDeque<u32> {
    VecDeque::from(si
        .strip_prefix("  Starting items:")
        .unwrap()
        .split(",")
        .map(|i| i.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>())
}

fn construct_operation_from_str<'a>(op: &str) -> Box<dyn Fn(u32) -> u32> {
    let op_parts: Vec<&str> = op
        .strip_prefix("  Operation: new = ")
        .unwrap()
        .splitn(3, " ")
        .collect();
    
    return if op_parts[2] == "old" {
        if op_parts[1] == "+" {
            Box::new(|v| v + v)
        } else {
            Box::new(|v| v * v)
        }
    } else {
        let second_value: u32 = op_parts[2].parse().unwrap();
        if op_parts[1] == "+" {
            Box::new(move |v| v + second_value)
        } else {
            Box::new(move |v| v * second_value)
        }
    }
}

fn construct_monkey_test_from_str_vec<'a>(test_instructions: Vec<&str>) -> Box<dyn Fn(u32) -> usize> {
    let divisible_by: u32 = test_instructions[0].strip_prefix("  Test: divisible by ").unwrap().parse().unwrap();
    let if_true: usize =  test_instructions[1].strip_prefix("    If true: throw to monkey ").unwrap().parse().unwrap();
    let if_false: usize = test_instructions[2].strip_prefix("    If false: throw to monkey ").unwrap().parse().unwrap();
    
    fn divisible_by_func(v: u32, divisible_by: u32, if_true: usize, if_false: usize) -> usize {
        if v % divisible_by == 0 {
            if_true
        } else {
            if_false
        }
    }
    
    return Box::new(move |v| divisible_by_func(v, divisible_by, if_true, if_false));
}

struct Monkey<'a> {
    starting_items: VecDeque<u32>, 
    operation: Box<dyn Fn(u32) -> u32+'a>,
    test: Box<dyn Fn(u32) -> usize + 'a>,
}

impl<'a> Monkey<'a> {
    fn new(
        starting_items: VecDeque<u32>,
        operation: impl Fn(u32) -> u32 + 'a,
        test: impl Fn(u32) -> usize + 'a
    ) -> Self {
        Self {
            starting_items,
            operation: Box::new(operation),
            test: Box::new(test),
        }
    }
    
    fn dequeue(&mut self) -> u32 {
        self.starting_items.pop_front().unwrap()
    }
    
    fn enqueue(&mut self, item: u32) {
        self.starting_items.push_back(item)
    }
}