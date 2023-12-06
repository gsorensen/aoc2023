fn count_winning_combinations(duration: usize, record: usize) -> usize {
    (1..duration)
        .filter(|time| time * (duration - time) > record)
        .count()
}

fn main() -> Result<(), anyhow::Error> {
    let puzzle_input = std::fs::read_to_string("inputs/day6.txt")?;

    let mut parsed_input = puzzle_input.split("\n").take(2).map(|x| {
        x.split(":")
            .nth(1)
            .expect("The raw numbers should exist")
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<usize>>()
    });

    let races = parsed_input
        .nth(0)
        .expect("Times should exist")
        .into_iter()
        .zip(parsed_input.nth(0).expect("Records should exist"))
        .collect::<Vec<(usize, usize)>>();

    let winning_combos_product = races
        .iter()
        .map(|(duration, record)| count_winning_combinations(*duration, *record))
        .fold(1, |acc, x| acc * x);

    println!("Winning combinations product: {winning_combos_product}");

    let parsed_input = puzzle_input
        .split("\n")
        .take(2)
        .filter_map(|x| {
            x.split(":")
                .nth(1)
                .expect("The raw numbers should exist")
                .split_whitespace()
                .collect::<String>()
                .parse::<usize>()
                .ok()
        })
        .collect::<Vec<usize>>();

    let (duration, record) = (parsed_input[0], parsed_input[1]);

    let winning_combo = count_winning_combinations(duration, record);

    println!("Winning combinations for big race: {}", winning_combo);

    Ok(())
}
