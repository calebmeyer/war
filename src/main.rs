fn main() {
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
    let ranks = [
        Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven,
        Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King,
        Rank::Ace,
    ];

    enum Suit {
        Spades,
        Hearts,
        Clubs,
        Diamonds,
    }
    let suits = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds,];

    struct Card {
        rank: Rank,
        suit: Suit,
    }

    let mut deck: Vec<Card> = Vec::new();

    for suit in suits.iter() {
        for rank in ranks.iter() {
            deck.push(Card{rank, suit});
        }
    }
}
