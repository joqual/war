use war::{Card, Deck};

fn main() {
    println!("Building decks ...");

    let mut hero_deck: Deck = Deck::build();

    // Clone deck created for 
    let mut villain_deck: Deck = Deck::build();

    // Shuffle hero and villain decks
    Deck::shuffle(&mut hero_deck.cards);
    Deck::shuffle(&mut villain_deck.cards);

    println!("Starting War!");

    let mut pile: Vec<Card> = Vec::new();

    // Enter game loop
    'game: loop {

        let hero_draw: Card = hero_deck.cards.pop().unwrap();
        println!("Hero Drew: {:?}", hero_draw);

        let villain_draw: Card = villain_deck.cards.pop().unwrap();
        println!("Villain Drew: {:?}", villain_draw);

        pile.push(hero_draw);
        pile.push(villain_draw);

        if villain_draw.value > hero_draw.value {
            println!("Villain wins the round!");
            for c in &pile {
                villain_deck.cards.insert(0, *c);
            }
            pile.clear();
        } else if hero_draw.value > villain_draw.value {
            println!("Hero wins the round!");
            for c in &pile {
                hero_deck.cards.insert(0, *c);
            }
            pile.clear();
        } else {
            println!("It's a tie! War!");

            // Handle war scenario
            'warloop: for _ in 0..2 {
                // Blindly push 3 cards from each deck into pile to be collected
                if hero_deck.is_empty() | villain_deck.is_empty() {
                    break 'warloop;
                }
                pile.push(hero_deck.cards.pop().unwrap());
                pile.push(villain_deck.cards.pop().unwrap());
                
                // and continue as normal
            }

            count += 1;
        }

        // win condition
        if hero_deck.is_empty() | villain_deck.is_empty() {
            println!("Game Over!");
            if hero_deck.is_empty() {
                println!("Hero Wins!");
            } else {
                println!("Villain Wins!")
            }
            break 'game;
        }
    }
}