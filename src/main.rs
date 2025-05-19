use std::process;

// DO NOT CHANGE THESE VALUES
static DECK_LENGTH: u8 = 52;
static NUM_VALUES: u8 = 13;

fn main() {
    println!("The War has begun.");

    println!("Press enter to draw.");

    let my_deck: Deck = match Deck::build() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Error: failed to build deck.");
            process::exit(-1);
        }
    };

    println!("{:?}", my_deck.cards[0])
}

#[derive(Debug)]
enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Value {
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

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn build() -> Result<Deck, &'static str> {
        println!("Build the deck ...");

        let cards = Vec::new();

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

            let val: Value = match i % NUM_VALUES  {
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

            
            println!("{i}, {:?}, {:?}", suit, val);

        }

        Ok(Deck { cards })
    }
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    value: Value,
}
