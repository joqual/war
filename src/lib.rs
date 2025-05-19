use rand::seq::SliceRandom;
use std::process;

// DO NOT CHANGE THESE VALUES
static DECK_LENGTH: u8 = 52;
static NUM_VALUES: u8 = 13;

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn build() -> Result<Deck, &'static str> {
        let mut cards = Vec::new();

        for i in 0..DECK_LENGTH {
            let suit: Suit = match i {
                0..13 => Suit::Club,
                13..26 => Suit::Diamond,
                26..39 => Suit::Heart,
                39..52 => Suit::Spade,
                _ => {
                    eprintln!("Error building deck");
                    process::exit(-1);
                }
            };

            let val: Value = match i % NUM_VALUES {
                0 => Value::Two,
                1 => Value::Three,
                2 => Value::Four,
                3 => Value::Five,
                4 => Value::Six,
                5 => Value::Seven,
                6 => Value::Eight,
                7 => Value::Nine,
                8 => Value::Ten,
                9 => Value::Jack,
                10 => Value::Queen,
                11 => Value::King,
                12 => Value::Ace,
                _ => {
                    eprintln!("Error building deck");
                    process::exit(-1);
                }
            };

            cards.push(Card {
                suit: suit,
                value: val,
            });
        }

        Ok(Deck { cards })
    }

    pub fn shuffle(cards: &mut Vec<Card>) {
        let mut rng = rand::rng();
        cards.shuffle(&mut rng)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}
