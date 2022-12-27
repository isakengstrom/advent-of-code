fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");
    
    let tree_grid: Vec<Vec<u16>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u16).collect())
        .collect();
    
    let edge_count = tree_grid.len() * 2 + tree_grid[0].len() * 2 - 4;

    let mut interior_count = 0;
    for (row_index, tree_row) in tree_grid.iter().enumerate() {
        if row_index <= 0 || row_index >= tree_grid.len()-1 {
            continue;
        }
        
        for (col_index, curr_tree_height) in tree_row.iter().enumerate() {
            if col_index <= 0 || col_index >= tree_row.len()-1 {
                continue;
            }

            let has_block_to_left = tree_grid[row_index][0..col_index].iter().any(|v| *v >= *curr_tree_height);
            let has_block_to_right = tree_grid[row_index][col_index+1..=tree_row.len()-1].iter().any(|v| *v >= *curr_tree_height);
            
            let has_block_to_top = tree_grid[0..row_index].iter().map(|f| f[col_index]).any(|v| v >= *curr_tree_height);
            let has_block_to_bottom = tree_grid[row_index+1..=tree_grid.len()-1].iter().map(|f| f[col_index]).any(|v| v >= *curr_tree_height);

            if !(has_block_to_left && has_block_to_right && has_block_to_top && has_block_to_bottom) {
                interior_count += 1;
            }
        }
    }

    let total_count = edge_count + interior_count;

    println!("{total_count}")
}
