use std::collections::HashMap;
use std::fmt;

fn main() {
    let opponent_representations: HashMap<&str, GameOption> = HashMap::from([
        ("A", GameOption::Rock),
        ("B", GameOption::Paper),
        ("C", GameOption::Scissor)
    ]);

    let player1_representations: HashMap<&str, u32> = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6)
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

        total_score += player1_selection;
        
        if player1_selection == &3 {
            total_score += get_representation_score(opponent_selection);
        }
        else if player1_selection == &0 {
            total_score += get_loosing_representation_score(opponent_selection);
        }
        else {
            total_score += get_winning_representation_score(opponent_selection);
        }
    }

    println!("{total_score}")
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

fn get_loosing_representation_score(representation: &GameOption) -> u32 {
    return if representation == &GameOption::Rock {
        3
    } else if representation == &GameOption::Paper {
        1
    } else {
        2
    }
}

fn get_winning_representation_score(representation: &GameOption) -> u32 {
    return if representation == &GameOption::Rock {
        2
    } else if representation == &GameOption::Paper {
        3
    } else {
        1
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
