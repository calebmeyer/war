mod deck;
use deck::Deck;

fn main() {
    let mut mydeck = Deck{ cards: Vec::with_capacity(52) };
    mydeck.init();

    let hand = mydeck.deal(5);
    let hand2 = mydeck.deal(5);

    println!("{}", hand);
    println!("{}", hand.cards.first().unwrap());
    println!("{}", hand2);
}
