mod deck;
use deck::Deck;


fn main() {
    let mut full_deck = Deck{ cards: Vec::with_capacity(52) };
    full_deck.init();

    let mut state = war::start(full_deck);
    for n in 1..=10 {
        state = war::turn(state);
    }
}

mod war {
    use crate::deck::Deck;

    #[derive(Debug)]
    pub struct GameState {
        pub my_deck: Deck,
        pub your_deck: Deck,
    }

    pub fn start(mut deck: Deck) -> GameState {
        let mut my_deck = deck.deal(26);
        let mut your_deck = deck.deal(26);

        GameState{ my_deck, your_deck }
    }

    pub fn turn(mut state: GameState) -> GameState {
        let my_card = state.my_deck.cards.remove(0);
        let your_card = state.your_deck.cards.remove(0);

        println!("My card: {}", my_card);
        println!("Your card: {}", your_card);

        if my_card.rank < your_card.rank {
            state.your_deck.cards.push(my_card);
            state.your_deck.cards.push(your_card);
        } else if my_card.rank > your_card.rank {
            state.my_deck.cards.push(my_card);
            state.my_deck.cards.push(your_card);
        } else {
            // TODO: Make this a proper war
            state.my_deck.cards.push(my_card);
            state.your_deck.cards.push(your_card);
        }

        println!("I have {} cards", state.my_deck.cards.len());
        println!("You have {} cards", state.your_deck.cards.len());
        state
    }
}
