use war::Deck;

fn main() {
    println!("Building decks ...");

    let mut hero_deck: Deck = Deck::build();

    // Clone deck created for 
    let mut villain_deck: Deck = Deck::build();

    // Shuffle hero and villain decks
    Deck::shuffle(&mut hero_deck.cards);
    Deck::shuffle(&mut villain_deck.cards);
}
