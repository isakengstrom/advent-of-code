
fn part_1(input: &str) -> u32 {
    let patterns = parse(input);

    let mut solution: u32 = 0;
    for pattern_versions in patterns {
        // Will only have a value representing the number of cols if there is a vertical reflection, otherwise 0
        let vertical_reflection_solution = try_find_reflection(&pattern_versions.1) as u32;

        // Will only have a value representing the number of rows if there is a horizontal reflection, otherwise 0
        let horizontal_reflection_solution = try_find_reflection(&pattern_versions.0) as u32;

        solution += horizontal_reflection_solution * 100 + vertical_reflection_solution;
    }

    solution
}

fn part_2(input: &str) -> u32 {
    let patterns = parse(input);

    let mut solution: u32 = 0;
    for (index, pattern_versions) in patterns.iter().enumerate() {

        // Will only have a value representing the number of cols if there is a vertical reflection, otherwise 0
        let vertical_reflection_solution = try_find_reflection_with_smudge(&pattern_versions.1) as u32;

        if vertical_reflection_solution > 0 {
            solution += vertical_reflection_solution;
            continue
        }

        // Will only have a value representing the number of rows if there is a horizontal reflection, otherwise 0
        let horizontal_reflection_solution = try_find_reflection_with_smudge(&pattern_versions.0) as u32;

        if horizontal_reflection_solution > 0 {
            solution += horizontal_reflection_solution * 100;
            continue
        }


        println!("No vertical or horizontal smudge solution for index {}",index)
    }

    solution
}

fn parse(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let parsed = input
        .trim()
        .split("\r\n\r\n")
        .map(|pattern| {
            let original_pattern: Vec<Vec<_>> = pattern.lines().map(|line| line.chars().collect()).collect();

            let flipped_pattern: Vec<Vec<_>> = (0..original_pattern[0].len())
                .map(|i| original_pattern.iter().map(|row| row[i]).collect())
                .collect();

            (original_pattern.iter().map(|row| row.iter().collect()).collect(), flipped_pattern.iter().map(|row| row.iter().collect()).collect())
        })
        .collect();

    parsed
}

/// Loops through and looks for the center line of reflection. Once a potential line is found, take a step in both directions from the line,
/// to verify that all lines actually reflect based on that center line of reflection.
///
/// Returns a positive value if a true reflection was found, otherwise 0.
fn try_find_reflection(pattern: &[String]) -> usize {
    let num_lines = pattern.len();

    // Loop through and look for the center line of reflection
    for index in 0..num_lines - 1 {
        // If the two lines of the pattern aren't equal, they don't create the line of reflection together.
        // In that case, continue on and check the next combo of lines
        if pattern[index] != pattern[index + 1] {
            continue;
        }

        // If reached and condition is true, there is a potential line of reflection between line indices 0 and 1.
        // This potential line of reflection will always be the true line of reflection, so no need to step.
        // Instead, return the one-based index.
        if index == 0 {
            return 1
        }

        // If reached and condition is true, there is a potential line of reflection between line indices next to last and last.
        // This potential line of reflection will always be the true line of reflection, so no need to step.
        // Instead, return the one-based index.
        if index + 1 == num_lines - 1 {
            return index + 1
        }

        // If reached, a potential center line of reflection has been found.
        // Take steps in both directions from the center line
        let max_steps = (index + 1).min(num_lines - index - 1);
        for step in 0..max_steps {
            let line_on_one_side_of_center = &pattern[index - step];
            let line_on_other_side_of_center = &pattern[index + 1 + step];

            // If the lines aren't equal, the potential line of reflection is false.
            // Break and continue looking for another center line of reflection
            if line_on_one_side_of_center != line_on_other_side_of_center {
                break;
            }

            let solution_is_found = step + 1 == max_steps;
            if solution_is_found {
                // Convert to one-based index (as per the task description),
                // representing the number of columns to the left, or rows above the line of reflection
                return index + 1;
            }
        }
    }
    0
}

/// Loops through and looks for the center line of reflection. Once a potential line is found, take a step in both directions from the line,
/// to verify that all lines actually reflect based on that center line of reflection.
///
/// Returns a positive value if a true reflection was found, otherwise 0.
fn try_find_reflection_with_smudge(pattern: &[String]) -> usize {
    let num_lines = pattern.len();

    // Loop through and look for the center line of reflection
    for index in 0..num_lines - 1 {
        let mut smudged = false;

        let first_line_index = index;
        let other_line_index = index + 1;

        let first_reflection_line = pattern[first_line_index].as_str();
        let other_reflection_line = pattern[other_line_index].as_str();

        // If the two lines of the pattern aren't equal, they don't create the line of reflection together.
        // In that case, continue on and check the next combo of lines
        if first_reflection_line != other_reflection_line {
            if !has_single_smudge(first_reflection_line, other_reflection_line){
                continue;
            }
            smudged = true
        }

        // If reached and condition is true, there is a potential line of reflection between line indices 0 and 1.
        // This potential line of reflection will always be the true line of reflection, so no need to step.
        // Instead, return the one-based index.
        // Smudge update: there needs to be a single smudge added to the pattern in order for it to be a solution
        if smudged && first_line_index == 0 {
            return first_line_index + 1
        }

        // If reached and condition is true, there is a potential line of reflection between line indices next to last and last.
        // This potential line of reflection will always be the true line of reflection, so no need to step.
        // Instead, return the one-based index.
        // Smudge update: there needs to be a single smudge added to the pattern in order for it to be a solution
        if smudged && other_line_index == num_lines - 1 {
            return first_line_index + 1
        }

        // If reached, a potential center line of reflection has been found.
        // Take steps in both directions from the center line
        let max_steps = (index + 1).min(num_lines - index - 1);
        for step in 1..max_steps {
            let line_on_one_side_of_center = &pattern[first_line_index - step].as_str();
            let line_on_other_side_of_center = &pattern[other_line_index + step].as_str();

            // If the lines aren't equal, the potential line of reflection is false.
            // Break and continue looking for another center line of reflection
            if line_on_one_side_of_center != line_on_other_side_of_center {
                if smudged {
                    break;
                }

                if !has_single_smudge(line_on_one_side_of_center, line_on_other_side_of_center) {
                    break;
                }

                smudged = true
            }

            let smudged_solution_is_found = smudged && step + 1 == max_steps;
            if smudged_solution_is_found {
                // Convert to one-based index (as per the task description),
                // representing the number of columns to the left, or rows above the line of reflection.
                // Smudge update: there needs to be a single smudge added to the pattern in order for it to be a solution.
                return first_line_index + 1;
            }
        }
    }
    0
}

fn has_single_smudge(line_one: &str, line_two: &str) -> bool {

    let mut smudges: u16 = 0;

    for (tile_1, tile_2) in line_one.chars().zip(line_two.chars()) {
        if tile_1 == tile_2 {
            continue;
        }

        smudges += 1;

        // If more than one smudge is found, there cannot be only one smudge
        if smudges > 1 {
            return false;
        }
    }

    // Only one smudge is allowed
    smudges == 1
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use test_case::test_case;

    #[test_case("input.example.0.txt", 405)]
    #[test_case("input.example.1.txt", 709)]
    fn part_1_output_should_be_correct(input_path: &str, expected: u32)
    {
        // Arrange
        let input_0 = fs::read_to_string(input_path).expect("Failed to read file");
        let input = input_0.as_str();

        // Act
        let result = part_1(input);

        // Assert
        assert_eq!(result, expected)
    }

    #[test_case("input.example.0.txt", 400)]
    #[test_case("input.example.1.txt", 1400)]
    fn part_2_output_should_be_correct(input_path: &str, expected: u32)
    {
        // Arrange
        let input_0 = fs::read_to_string(input_path).expect("Failed to read file");
        let input = input_0.as_str();

        // Act
        let result = part_2(input);

        // Assert
        assert_eq!(result, expected)
    }
}