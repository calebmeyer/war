mod rank;
use rank::Rank;

mod suit;
use suit::Suit;

mod card;
use card::Card;

mod deck;
use deck::Deck;

fn main() {
    let mut deck = Deck{cards: Vec::with_capacity(52)};

    for &suit in Suit::variants() {
        for &rank in Rank::variants() {
            deck.cards.push(Card{rank, suit});
        }
    }

    println!("{:?}", deck);
    println!("{}", deck.cards.first().unwrap());
}
