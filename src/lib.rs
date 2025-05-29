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

impl Suit {
    const ALL: [Suit; 4] = [Suit::Heart, Suit::Spade, Suit::Diamond, Suit::Club];
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

impl Suit {
    const ALL: [Suit; 4] = [Suit::Heart, Suit::Spade, Suit::Diamond, Suit::Club];
}

impl Value {
    const ALL: [Value; 13] = [
        Value::Two,
        Value::Three,
        Value::Four,
        Value::Five,
        Value::Six,
        Value::Seven,
        Value::Eight,
        Value::Nine,
        Value::Ten,
        Value::Jack,
        Value::Queen,
        Value::King,
        Value::Ace,
    ];
}

impl Deck {
    pub fn build() -> Result<Deck, &'static str> {
        let mut cards = Vec::new();

        for &suit in Suit::ALL.iter() {
            for &value in Value::ALL.iter() {
                cards.push(Card { suit, value });
            }
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
