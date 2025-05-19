use std::process;
use war::Deck;

fn main() {
    println!("The War has begun.");

    println!("Press enter to draw.");

    let my_deck: war::Deck = match Deck::build() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Error: failed to build deck.");
            process::exit(-1);
        }
    };

    println!("{:?}", my_deck.cards[0])
}