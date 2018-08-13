use std::fmt;

#[derive(Copy, Clone, Debug)]
enum Rank {
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

impl Rank {
    fn variants() -> std::slice::Iter<'static, Rank> {
        [
            Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
            Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack,
            Rank::Queen, Rank::King, Rank::Ace,
        ].iter()
    }
}

#[derive(Copy, Clone, Debug)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl Suit {
    fn variants() -> std::slice::Iter<'static, Suit> {
        [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds,].iter()
    }
}

#[derive(Copy, Clone, Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

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
