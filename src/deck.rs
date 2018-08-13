extern crate rand;

use card::Card;
use self::rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn shuffle(&mut self) {
        thread_rng().shuffle(self.cards.as_mut_slice());
    }

    // removes the first number_of_cards from deck and returns them as a new Hand
    pub fn deal(&mut self, number_of_cards: usize) -> Hand {
        let drain = self.cards.drain(..number_of_cards);
        Hand{cards: drain.collect()}
    }
}

type Hand = Deck; // a hand of cards is really just a smaller deck
type DiscardPile = Deck; // A discard pile is just a faceup deck
