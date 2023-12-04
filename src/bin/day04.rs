use rayon::prelude::*;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    player_numbers: Vec<u32>,
}

impl Card {
    fn get_player_winning_numbers(&self) -> Vec<u32> {
        self.player_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(&number))
            .map(|r| *r)
            .collect()
    }

    fn compute_points(&self) -> u32 {
        self.get_player_winning_numbers()
            .iter()
            .fold(0, |acc, _| match acc {
                0 => 1,
                x => 2 * x,
            })
    }

    fn get_copies_from_win(&self, scratch_cards: &Vec<Card>) -> Vec<Card> {
        (self.id + 1..=self.id + self.get_player_winning_numbers().len() as u32)
            .filter_map(|id| match scratch_cards.iter().find(|card| card.id == id) {
                Some(card) => Some(card.clone()),
                _ => None,
            })
            .collect::<Vec<Card>>()
    }

    fn get_copies_total(&self, scratch_cards: &Vec<Card>) -> u32 {
        let copies = self.get_copies_from_win(&scratch_cards);
        let total = copies.iter().fold(copies.len() as u32, |acc, copy| {
            acc + copy.get_copies_total(&scratch_cards)
        });

        total
    }
}

impl TryFrom<&str> for Card {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split(":");

        let id = match iter.next() {
            Some(id_str) => u32::from_str_radix(&id_str.replace("Card ", "").trim(), 10)
                .map_err(|_| Error::new(ErrorKind::Other, "Couldn't parse id")),
            None => Err(Error::new(
                ErrorKind::Other,
                "Unknown iterator contents when parsing ID",
            )),
        }?;

        let (winning_numbers, player_numbers) = match iter.next() {
            Some(numbers_str) => {
                let (winning_numbers_str, player_numbers_str) = numbers_str
                    .split_once("|")
                    .ok_or_else(|| Error::new(ErrorKind::Other, "Couldn't split"))?;

                let winning_numbers = winning_numbers_str
                    .trim()
                    .replace("  ", " ")
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| u32::from_str_radix(s, 10).ok())
                    .collect::<Vec<u32>>();

                let player_numbers = player_numbers_str
                    .trim()
                    .replace("  ", " ")
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| u32::from_str_radix(s, 10).ok())
                    .collect::<Vec<u32>>();

                Ok((winning_numbers, player_numbers))
            }
            None => Err(Error::new(
                ErrorKind::Other,
                "Unknown iterator contents when parsing numbers",
            )),
        }?;

        Ok(Self {
            id,
            winning_numbers,
            player_numbers,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = std::fs::read_to_string("inputs/day4.txt")?;

    let cards = puzzle_input
        .lines()
        .filter_map(|line| Card::try_from(line).ok())
        .collect::<Vec<Card>>();

    let points = cards.iter().map(|card| card.compute_points()).sum::<u32>();

    let num_scratchcards = cards.len() as u32
        + cards
            .par_iter()
            .map(|card| card.get_copies_total(&cards))
            .sum::<u32>();

    println!(
        "Scratch card points: {}\nScratch card total with copies {}",
        points, num_scratchcards
    );

    Ok(())
}
