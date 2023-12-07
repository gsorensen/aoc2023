use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Ord, PartialEq, Eq)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl TryFrom<Vec<Card>> for Type {
    type Error = &'static str;

    fn try_from(value: Vec<Card>) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            return Err("Hands need to have length 5");
        }

        let counts = value.iter().counts();
        let most_occuring_card = counts
            .iter()
            .max_by(|x, y| {
                // If jokers are the most frequent card, we are not interested in having
                // that as the most occuring card
                if x.1 != y.1 && (x.0.strength != 1 && y.0.strength != 1) {
                    x.1.cmp(&y.1)
                } else {
                    x.0.cmp(&y.0)
                }
            })
            .expect("There are cards");
        let joker_free_counts = value
            .iter()
            .map(|card| {
                if card.strength == 1 {
                    most_occuring_card.clone().0
                } else {
                    card
                }
            })
            .counts();

        Ok(match joker_free_counts.len() {
            5 => Type::HighCard,
            4 => Type::OnePair,
            3 => {
                if joker_free_counts.values().any(|&x| x > 2) {
                    Type::ThreeOfAKind
                } else {
                    Type::TwoPair
                }
            } // Two pair or three of a kind
            2 => {
                if joker_free_counts.values().any(|&x| x > 3) {
                    Type::FourOfAKind
                } else {
                    Type::FullHouse
                }
            } // Four of a kind or full house
            1 => Type::FiveOfAKind,
            _ => unreachable!("Can't happen"),
        })
    }
}

#[derive(Debug, Eq, Hash, Copy, Clone)]
struct Card {
    label: char,
    strength: u32,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        let label = value;

        // NOTE: This isn't proper error handling, but it's a puzzle
        let strength = match label {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            '2'..='9' => label.to_digit(10).expect("Should work"),
            _ => panic!("Invalid input"),
        };

        Self { label, strength }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.strength.cmp(&other.strength)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Eq)]
struct Hand {
    // NOTE: Cards are always five length, but vecs are easier to deal with than fixed-size arrays
    cards: Vec<Card>,
    bid_amount: u32,
    hand_type: Type,
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let cards = iter
            .next()
            .ok_or("Invalid input")?
            .chars()
            .map(|c| Card::from(c))
            .collect::<Vec<_>>();

        // Laziness cloning
        let hand_type = Type::try_from(cards.clone())?;

        let bid_amount = iter
            .next()
            .ok_or("Invalid input")?
            .parse::<u32>()
            .map_err(|_| "Failed to parse bid amount")?;

        Ok(Self {
            cards,
            bid_amount,
            hand_type,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }

        self.cards
            .iter()
            .zip(&other.cards)
            .all(|(card, other_card)| card == other_card)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (card, other_card) in self.cards.iter().zip(&other.cards) {
                if card == other_card {
                    continue;
                }

                return card.cmp(&other_card);
            }
        }

        // Sort is wrong when case is identical
        self.hand_type.cmp(&other.hand_type)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn main() -> Result<(), anyhow::Error> {
    let puzzle_input = std::fs::read_to_string("inputs/day7.txt")?;

    let mut hands = puzzle_input
        .split("\n")
        .into_iter()
        .filter_map(|line| Hand::from_str(line).ok())
        .collect::<Vec<_>>();

    hands.sort();

    for hand in &hands {
        println!("{:?}", hand.hand_type);
    }

    let total_winnings = hands.iter().enumerate().fold(0, |acc, (idx, hand)| {
        acc + (idx + 1) * hand.bid_amount as usize
    });

    println!("Total winnings: {total_winnings}");

    Ok(())
}
