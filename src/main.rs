use std::process;
use war::Deck;

fn main() {
    println!("Building decks ...");

    let mut hero_deck: Deck = match Deck::build() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Error: failed to build deck.");
            process::exit(-1);
        }
    };

    // Clone deck created
    let mut villain_deck: Deck = hero_deck.clone();

    // Shuffle hero and villain decks
    Deck::shuffle(&mut hero_deck.cards);
    Deck::shuffle(&mut villain_deck.cards);
}
