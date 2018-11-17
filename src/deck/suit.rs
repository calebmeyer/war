use std;

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl Suit {
    pub fn variants() -> std::slice::Iter<'static, Suit> {
        [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds,].iter()
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Suit::Spades => write!(f, "♠"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Clubs => write!(f, "♣"),
            Suit::Diamonds => write!(f, "♦"),
        }
    }
}
