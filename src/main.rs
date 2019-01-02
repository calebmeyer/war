mod deck;
use deck::Deck;


fn main() {
    let mut full_deck = Deck{ cards: Vec::with_capacity(52) };
    full_deck.init();

    let mut state = war::start(full_deck);
    state = war::turn(state);
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
        println!("{}", my_card);
        println!("{}", your_card);

        state
    }
}
