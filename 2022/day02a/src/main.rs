use std::collections::HashMap;
use std::fmt;

fn main() {
    let opponent_representations: HashMap<&str, GameOption> = HashMap::from([
        ("A", GameOption::Rock),
        ("B", GameOption::Paper),
        ("C", GameOption::Scissor)
    ]);
    
    let player1_representations: HashMap<&str, GameOption> = HashMap::from([
        ("X", GameOption::Rock),
        ("Y", GameOption::Paper),
        ("Z", GameOption::Scissor)  
    ]);
    
    let mut input = include_str!("../input.txt").to_string();

    // input may have a disgusting bom char in the beginning. If so, remove it  
    if input.starts_with('\u{feff}') {
        input.remove(0);
    }
    
    let rounds = input.lines().collect::<Vec<&str>>();

    let mut total_score: u32 = 0;
    
    for round in rounds {
        let selections: Vec<&str> = round.split_whitespace().collect();
        let opponent_selection = opponent_representations.get(selections[0]).unwrap();
        let player1_selection = player1_representations.get(selections[1]).unwrap();
        
        let player1_representation_score = get_representation_score(player1_selection);
        let player1_result_score = get_result_score(opponent_selection,player1_selection);
        
        total_score += player1_representation_score + player1_result_score;
    }
    
    println!("{total_score}")
}

fn get_result_score(opponent_selection: &GameOption, player1_selection: &GameOption) -> u32 {
    if opponent_selection == player1_selection {
        return 3;
    }
    
    if  (player1_selection == &GameOption::Rock && opponent_selection == &GameOption::Scissor) || 
        (player1_selection == &GameOption::Paper && opponent_selection == &GameOption::Rock) ||
        (player1_selection == &GameOption::Scissor && opponent_selection == &GameOption::Paper) {
        return 6;
    }
    
    return 0;
}

fn get_representation_score(representation: &GameOption) -> u32 {
    return if representation == &GameOption::Rock {
        1
    } else if representation == &GameOption::Paper {
        2
    } else {
        3
    }
}

// To enable printing for enum
impl fmt::Display for GameOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameOption::Rock => write!(f, "Rock"),
            GameOption::Paper => write!(f, "Paper"),
            GameOption::Scissor => write!(f, "Scissor")
        }
    }
}

#[derive(Eq, PartialEq)]
enum GameOption {
    Rock,
    Paper,
    Scissor
}


