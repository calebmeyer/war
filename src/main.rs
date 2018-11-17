mod deck;
use deck::Deck;

fn main() {
    let mut deck = Deck.new();

    let hand = deck.deal(5);
    let hand2 = deck.deal(5);

    println!("{}", hand);
    println!("{}", hand.cards.first().unwrap());
    println!("{}", hand2);
}
