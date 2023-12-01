use anyhow::Error;

fn locate_number(input: &str) -> Vec<(usize, usize)> {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut indices = vec![];

    for number in numbers {
        let number_str = format!("{}", number);

        // Find first occurence
        match input.find(&number_str) {
            Some(idx) => indices.push((idx, number)),
            None => (),
        }

        // Find last occurence
        match input.rfind(&number_str) {
            Some(idx) => indices.push((idx, number)),
            None => (),
        }
    }

    indices
}

fn locate_number_literals(input: &str) -> Vec<(usize, usize)> {
    let number_pairs = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut indices: Vec<(usize, usize)> = vec![];

    for (s, n) in number_pairs {
        // Find first occurence
        match input.find(s) {
            Some(idx) => indices.push((idx, n)),
            None => (),
        }

        // Find last occurence
        match input.rfind(s) {
            Some(idx) => indices.push((idx, n)),
            None => (),
        }
    }

    indices
}

fn sum_first_and_last_digit(input: &str, find_literal_numbers: bool) -> Option<usize> {
    let mut indices = locate_number(input);

    // Puzzle solution for part 1 and 2 differ based on whether you parse
    // written numbers
    if find_literal_numbers {
        let literal_indices = locate_number_literals(input);
        indices.extend(literal_indices);
    }

    indices.sort();
    let (_, first_number) = indices.first()?;
    let (_, last_number) = indices.last()?;

    let num_string = format!("{}{}", first_number, last_number);

    match usize::from_str_radix(&num_string, 10) {
        Ok(val) => Some(val),
        _ => None,
    }
}

fn main() -> Result<(), Error> {
    let puzzle_input = std::fs::read_to_string("inputs/day1.txt")?;

    let first_calibration_sum: usize = puzzle_input
        .split("\n")
        .filter(|line| line.len() > 0)
        .filter_map(|line| sum_first_and_last_digit(line, false))
        .sum();

    let second_calibration_sum: usize = puzzle_input
        .split("\n")
        .filter(|line| line.len() > 0)
        .filter_map(|line| sum_first_and_last_digit(line, true))
        .sum();

    println!(
        "First calibration sum: {}\nSecond calibration sum: {}",
        first_calibration_sum, second_calibration_sum
    );

    Ok(())
}
