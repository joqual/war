use std::process;
use std::mem;

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

        for i in 1..DECK_LENGTH+1 {

            let suit: Suit = match i {
                0..13 => Suit::Club,
                14..26 => Suit::Diamond,
                27..39 => Suit::Heart,
                40..52 => Suit::Spade,
            };

            println!("{i}");


            let
            if i % NUM_VALUES
        }

        Ok(Deck { cards })
    }
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    value: Value,
}
