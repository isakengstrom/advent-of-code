use std::collections::HashSet;

fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let motions = parse_motions(input);

    let tail_visited_positions = simulate(motions);
    
    println!("{}", tail_visited_positions.len());
    let i = 0;
}



fn simulate_with_display(motions: Vec<MotionType>) -> (HashSet<String>, Vec<(i32, i32)>) {
    let mut head = Position::origin();
    let mut tail = Position::origin();

    let mut tail_visited_positions: HashSet<String> = HashSet::new();
    let mut tail_visited_order: Vec<(i32, i32)> = Vec::new();
    
    let mut grid_min_x = 0;
    let mut grid_max_x = 0;
    let mut grid_min_y = 0;
    let mut grid_max_y = 0;
    
    for head_motion in motions {
        tail = step_tail(tail, head, head_motion);
        head.step(head_motion);
        tail_visited_positions.insert(tail.to_string());
        tail_visited_order.push((tail.x, tail.y));
        
        if tail.x < grid_min_x { grid_min_x = tail.x; }
        if tail.x > grid_max_x { grid_max_x = tail.x; }
        if tail.y < grid_min_y { grid_min_y = tail.y; }
        if tail.y > grid_max_y { grid_max_y = tail.y; }
    }
    
    let x_count = grid_max_x - grid_min_x;
    let y_count = grid_max_y - grid_min_y;

    
    
    for x_axis in grid_min_x..grid_max_x {
        
    }

    return (tail_visited_positions, tail_visited_order);
}

fn simulate(motions: Vec<MotionType>) -> HashSet<String> {
    let mut head = Position::origin();
    let mut tail = Position::origin();

    let mut tail_visited_positions: HashSet<String> = HashSet::new();


    for head_motion in motions {
        tail = step_tail(tail, head, head_motion);
        head.step(head_motion);
        tail_visited_positions.insert(tail.to_string());
    }

    return tail_visited_positions;
}


fn step_tail(tail: Position, head: Position, head_motion: MotionType) -> Position {

    let mut head_new = head.clone();
    head_new.step(head_motion);
    
    let mut tail_new = tail.clone();
    
    
    if Position::is_touching(&tail, &head_new) {
        return tail;
    }

  
    let manhattan_dist = Position::manhattan_distance(head, tail);
    
    match manhattan_dist {
        1 => {
            *tail_new.step(head_motion)
        }
        2 => {
            head
        }
        _ => {
            panic!("The head and the tail should never be this far apart.")
        }
    }
}



fn parse_motions(unparsed: &str) -> Vec<MotionType> {
    let batched_motions = unparsed
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|m| (m.0.chars().next().unwrap(), m.1.parse::<u16>().unwrap()))
        .collect::<Vec<(char, u16)>>();


    let mut motions: Vec<MotionType> = Vec::new();
    for batched_motion in batched_motions {
        for _ in 0..batched_motion.1 {
            match batched_motion.0 {
                'L' => {
                    motions.push(MotionType::Left)
                }
                'R' => {
                    motions.push(MotionType::Right)
                }
                'U' => {
                    motions.push(MotionType::Up)
                }
                'D' => {
                    motions.push(MotionType::Down)
                }
                _ => {}
            }
        }
    }
    let i = 0;
    
    return motions;
}


#[derive(Clone, Copy)]
enum MotionType {
    Left,
    Right,
    Up,
    Down
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn origin() -> Position {
        Position {x: 0, y: 0}
    }
    
    //fn new(x: i32, y: i32) -> Position {
    //    Position { x, y }
    //}
    
    fn manhattan_distance(pos_1: Position, pos_2: Position) -> u16 {
         ((pos_1.x - pos_2.x).abs() + (pos_1.y - pos_2.y).abs()) as u16
    }

    fn is_touching(pos_1: &Position, pos_2: &Position) -> bool {
        (pos_1.x - pos_2.x).abs() <= 1 && (pos_1.y - pos_2.y).abs() <= 1
    }
    
    fn step(&mut self, motion: MotionType) -> &Self {
        match motion {
            MotionType::Left => {
                self.x = self.x - 1;
            }
            MotionType::Right => {
                self.x = self.x + 1;
            }
            MotionType::Up => {
                self.y = self.y + 1;
            }
            MotionType::Down => {
                self.y = self.y - 1;
            }
        }
        
        return self;
    }
    
    fn to_string(& self) -> String {
        
        self.x.to_string() + &*String::from(',') +  &*self.y.to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
/*

 4 ...........
 3 ...........
 2 ...........
 1 ...........
 0 .....H.....
-1 ...........
-2 ...........
-3 ...........
-4 ...........
   -----
   54321012345
 */
