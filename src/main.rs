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

#[derive(Copy, Clone, Debug)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
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

fn main() {
    let ranks = [
        Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven,
        Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King,
        Rank::Ace,
    ];

    let suits = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds,];

    let mut deck: Vec<Card> = Vec::new();

    for &suit in suits.iter() {
        for &rank in ranks.iter() {
            deck.push(Card{rank, suit});
        }
    }

    println!("{:?}", deck);
    println!("{}", deck.first().unwrap());
}
