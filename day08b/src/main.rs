fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let tree_grid: Vec<Vec<u16>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u16).collect())
        .collect();

    let mut max_scenic_score: u32 = 0;
    
    for (row_index, tree_row) in tree_grid.iter().enumerate() {
        if row_index <= 0 || row_index >= tree_grid.len()-1 {
            continue;
        }

        for (col_index, curr_tree_height) in tree_row.iter().enumerate() {
            if col_index <= 0 || col_index >= tree_row.len()-1 {
                continue;
            }

            let left_heights = tree_grid[row_index][0..col_index].iter().rev().map(|v| *v).collect::<Vec<u16>>();
            let right_heights = tree_grid[row_index][col_index+1..=tree_row.len()-1].iter().map(|v| *v).collect::<Vec<u16>>();

            let top_heights = tree_grid[0..row_index].iter().rev().map(|f| f[col_index]).collect::<Vec<u16>>();
            let bottom_heights = tree_grid[row_index+1..=tree_grid.len()-1].iter().map(|f| f[col_index]).collect::<Vec<u16>>();
            
            let left_scenic_score = get_scenic_direction_score(left_heights, *curr_tree_height);
            let right_scenic_score = get_scenic_direction_score(right_heights, *curr_tree_height);
            let top_scenic_score = get_scenic_direction_score(top_heights, *curr_tree_height);
            let bottom_scenic_score = get_scenic_direction_score(bottom_heights, *curr_tree_height);
            
            let scenic_score = left_scenic_score * right_scenic_score * top_scenic_score * bottom_scenic_score;
            
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    
    println!("{max_scenic_score}")
}

fn get_scenic_direction_score(heights: Vec<u16>, height_limit: u16) -> u32 {
    let mut count = 0;
    for height in heights {
        count += 1;
        if height >= height_limit {
            break;
        }
    }
    
    return count;
}