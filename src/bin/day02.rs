use anyhow::Error;

#[derive(Debug, PartialEq, Eq)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Colour {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            _ => Err("Invalid colour option"),
        }
    }
}

#[derive(Debug)]
struct CubeSet {
    colour: Colour,
    amount: usize,
}

impl TryFrom<&str> for CubeSet {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split(" ");

        let amount = usize::from_str_radix(iter.next().unwrap_or(""), 10)
            .map_err(|_| "Invalid amount string")?;
        let colour = Colour::try_from(iter.next().unwrap_or(""))?;

        Ok(CubeSet { amount, colour })
    }
}

#[derive(Debug)]
struct Round {
    sets: Vec<CubeSet>,
}

impl TryFrom<&str> for Round {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let iter = value.split(", ");

        let sets = iter
            .filter_map(|set_str| CubeSet::try_from(set_str).ok())
            .collect::<Vec<CubeSet>>();

        Ok(Round { sets })
    }
}

impl Round {
    pub fn max_seen_of(&self, colour: &Colour) -> usize {
        self.sets
            .iter()
            .filter(|set| &set.colour == colour)
            .map(|set| set.amount)
            .max()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    fn max_seen_of(&self, colour: &Colour) -> usize {
        self.rounds
            .iter()
            .map(|round| round.max_seen_of(colour))
            .max()
            .unwrap_or(0)
    }

    pub fn minimum_set_power(&self) -> usize {
        let max_red = self.max_seen_of(&Colour::Red);
        let max_blue = self.max_seen_of(&Colour::Blue);
        let max_green = self.max_seen_of(&Colour::Green);

        max_red * max_green * max_blue
    }

    pub fn is_possible_with(&self, contents: &BagContents) -> bool {
        self.max_seen_of(&Colour::Red) <= contents.red
            && self.max_seen_of(&Colour::Green) <= contents.green
            && self.max_seen_of(&Colour::Blue) <= contents.blue
    }
}

impl TryFrom<&str> for Game {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let trimmed_input = value.replace("Game ", "");
        let mut iter = trimmed_input.split(": ");

        let id = usize::from_str_radix(iter.next().unwrap_or(""), 10)
            .map_err(|_| "Couldn't parse ID")?;

        let rounds = iter
            .next()
            .unwrap_or("")
            .split("; ")
            .filter_map(|round_str| Round::try_from(round_str).ok())
            .collect::<Vec<_>>();

        Ok(Game { id, rounds })
    }
}

struct BagContents {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl BagContents {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        BagContents { red, green, blue }
    }
}

fn main() -> Result<(), Error> {
    let puzzle_input = std::fs::read_to_string("inputs/day2.txt")?;
    let contents = BagContents::new(12, 13, 14);

    let games = puzzle_input
        .split("\n")
        .filter_map(|line| Game::try_from(line).ok())
        .collect::<Vec<_>>();

    let id_sum = games
        .iter()
        .filter(|game| game.is_possible_with(&contents))
        .map(|game| game.id)
        .sum::<usize>();

    let power_sum = games
        .iter()
        .map(|game| game.minimum_set_power())
        .sum::<usize>();

    println!(
        "Sum of IDs of valid games: {}\nSum of minimum powers: {}",
        id_sum, power_sum
    );

    Ok(())
}
