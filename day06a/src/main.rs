use std::collections::HashSet;

fn main() {
    const MARKER_LENGTH: usize = 4;
    
    //let marker_start_index = include_str!("../input.example.txt")
    let marker_start_index = include_str!("../input.txt")
        .chars()
        .collect::<Vec<char>>()
        .windows(MARKER_LENGTH) 
        .skip(1)
        .position(|m| HashSet::<char>::from_iter(m.iter().map(|c| *c))
            .len() == MARKER_LENGTH
        ) // Find first occurrence when window has the correct number of uniques
        .unwrap();
    
    let marker_end_index = marker_start_index + MARKER_LENGTH;

    println!("{marker_end_index}");
}
