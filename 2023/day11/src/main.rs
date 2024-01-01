use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part_1(input.clone(), 2));
    println!("Part 2: {}", part_2(input.clone(), 1_000_000));
}

fn part_1(input: &str, expansion_scale: u64) -> u64 {
    let universe = parse_to_universe(input);
    let expanded_universe = expand_universe(universe, expansion_scale);
    calculate_distances_total(expanded_universe.0)
}

fn part_2(input: &str, expansion_scale: u64) -> u64 {
    let universe = parse_to_universe(input);
    let expanded_universe = expand_universe(universe, expansion_scale);
    calculate_distances_total(expanded_universe.0)
}

fn parse_to_universe(input: &str) -> (Vec<(u64, u64)>, usize, usize) {
    let lines = input.lines();

    let galaxy_sign = '#';
    let universe_height = lines.clone().count();
    let universe_width = lines.clone().into_iter().next().unwrap().len();

    let galaxies: Vec<(u64,u64)> = input
        .lines()
        .enumerate()
        .map(|(y_coord, line)|
            (y_coord, line
            .chars()
            .enumerate()
            .filter(|(_x_coord, sign_in_space)| *sign_in_space == galaxy_sign)
            .map(|(x_coord, _galaxy_sign)| x_coord)
            .collect::<Vec<_>>())
        )
        .flat_map(|(y_coord, x_galaxy_coords)| x_galaxy_coords
            .into_iter()
            .map(|x_coord| (x_coord as u64, y_coord as u64))
            .collect::<Vec<_>>()
        )
        .collect();

    (galaxies, universe_width, universe_height)
}

fn expand_universe(universe: (Vec<(u64, u64)>, usize, usize), expansion_scale: u64) -> (Vec<(u64, u64)>, usize, usize) {

    let mut galaxies = universe.0.clone();

    let universe_width = universe.1;
    let universe_height = universe.2;

    let unique_x_coords: HashSet<u64> = galaxies
        .iter()
        .map(|(x_coord, _y_coord)| *x_coord)
        .collect();
    let unique_y_coords: HashSet<u64> = galaxies
        .iter()
        .map(|(_x_coord, y_coord)| *y_coord)
        .collect();

    let mut x_coords_without_galaxies: Vec<u64> = (0..universe_width)
        .map(|i| i as u64)
        .filter(|&i| !unique_x_coords.contains(&i))
        .collect();
    x_coords_without_galaxies.sort_unstable_by(|a, b| b.cmp(a));
    x_coords_without_galaxies.dedup();

    let mut y_coords_without_galaxies: Vec<u64> = (0..universe_height)
        .map(|i| i as u64)
        .filter(|&i| !unique_y_coords.contains(&i))
        .collect::<Vec<u64>>();
    y_coords_without_galaxies.sort_unstable_by(|a, b| b.cmp(a));
    y_coords_without_galaxies.dedup();

    for x_coord_without_galaxies in &x_coords_without_galaxies {
        galaxies.iter_mut()
            .filter(|(x_coord, _)| x_coord > x_coord_without_galaxies)
            .for_each(|(x_coord, _)| *x_coord += expansion_scale - 1);
    }
    for y_coord_without_galaxies in &y_coords_without_galaxies {
        galaxies.iter_mut()
            .filter(|(_, y_coord)| y_coord > y_coord_without_galaxies)
            .for_each(|(_, y_coord)| *y_coord += expansion_scale - 1);
    }

    let expanded_width = universe.1 + x_coords_without_galaxies.len()*((expansion_scale - 1) as usize);
    let expanded_height =  universe.2 + y_coords_without_galaxies.len()*((expansion_scale - 1) as usize);

    (galaxies, expanded_width , expanded_height)
}

fn calculate_distances_total(galaxies: Vec<(u64, u64)>) -> u64 {
    let num_galaxies = galaxies.len();

    let mut manhattan_distances_total: u64 = 0;
    for i in 0..num_galaxies {
        for j in i+1..num_galaxies {
            let galaxy_a = galaxies[i];
            let galaxy_b = galaxies[j];

            let manhattan_distance  = galaxy_a.0.abs_diff(galaxy_b.0) + galaxy_a.1.abs_diff(galaxy_b.1);
            manhattan_distances_total += manhattan_distance
        }
    }
    manhattan_distances_total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_parse_and_expand_should_work_correctly()
    {
        // Arrange
        let input = include_str!("../input.example.0.txt");
        let input_expanded = include_str!("../input.example.0.expanded.txt");

        let universe  = parse_to_universe(input);

        let expected_expanded_universe = parse_to_universe(input_expanded);

        // Act
        let actual_expanded_universe = expand_universe(universe, 2);

        // Assert
        let actual_galaxies = actual_expanded_universe.0;
        let expected_galaxies = expected_expanded_universe.0;

        assert_eq!(actual_expanded_universe.1, expected_expanded_universe.1);
        assert_eq!(actual_expanded_universe.2, expected_expanded_universe.2);

        assert_eq!(actual_galaxies.len(), expected_galaxies.len());

        for i in 0..actual_galaxies.len() {
            let actual_x_coord = actual_galaxies[i].0;
            let actual_y_coord = actual_galaxies[i].1;
            let expected_x_coord = expected_galaxies[i].0;
            let expected_y_coord = expected_galaxies[i].1;

            assert_eq!(actual_x_coord, expected_x_coord);
            assert_eq!(actual_y_coord, expected_y_coord);
        }
    }

    #[test]
    fn part_1_output_should_be_correct()
    {
        // Arrange
        let input = include_str!("../input.example.0.txt");

        // Act
        let result = part_1(input, 2);

        // Assert
        assert_eq!(result, 374)
    }

    #[test]
    fn part_1_output_should_be_correct_1()
    {
        // Arrange
        let input = include_str!("../input.example.1.txt");

        // Act
        let result = part_1(input,2);

        // Assert
        assert_eq!(result, 10)
    }

    #[test]
    fn part_1_output_should_be_correct_2()
    {
        // Arrange
        let input = include_str!("../input.example.2.txt");

        // Act
        let result = part_1(input,2);

        // Assert
        assert_eq!(result, 8)
    }

    #[test]
    fn part_1_output_should_be_correct_3()
    {
        // Arrange
        let input = include_str!("../input.example.3.txt");

        // Act
        let result = part_1(input,2);

        // Assert
        assert_eq!(result, 40)
    }

    #[test]
    fn part_1_output_should_be_correct_4()
    {
        // Arrange
        let input = include_str!("../input.example.4.txt");

        // Act
        let result = part_1(input,2);

        // Assert
        assert_eq!(result, 2_466_269_413)
    }

    #[test]
    fn part_2_output_should_be_correct_for_10_scale()
    {
        // Arrange
        let input = include_str!("../input.example.0.txt");

        // Act
        let result = part_2(input, 10);

        // Assert
        assert_eq!(result, 1030)
    }

    #[test]
    fn part_2_output_should_be_correct_for_100_scale()
    {
        // Arrange
        let input = include_str!("../input.example.0.txt");

        // Act
        let result = part_2(input, 100);

        // Assert
        assert_eq!(result, 8410)
    }
}