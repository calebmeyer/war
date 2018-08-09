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

    enum Suit {
        Spades,
        Hearts,
        Clubs,
        Diamonds,
    }

    struct Card {
        rank: Rank,
        suit: Suit,
    }

    let mut deck: Vec<Card> = Vec::new();

    for suit in Suit.iter() {
        for rank in Rank.iter() {
            deck.push(Card{rank, suit});
        }
    }
}
