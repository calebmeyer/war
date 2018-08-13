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
