extern crate rand;

use std::fmt;
use self::rand::{thread_rng, Rng};

// mod rank;
use war::rank::Rank;

// mod ::suit;
use ::suit::Suit;

// mod ::card;
use super::card::Card;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck{cards: Vec::with_capacity(52)};

        for &suit in Suit::variants() {
            for &rank in Rank::variants() {
                deck.cards.push(Card{rank, suit});
            }
        }

        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        thread_rng().shuffle(self.cards.as_mut_slice());
    }

    // removes the first number_of_cards from deck and returns them as a new Hand
    pub fn deal(&mut self, number_of_cards: usize) -> Hand {
        let drain = self.cards.drain(..number_of_cards);
        Hand{cards: drain.collect()}
    }

    // gives cards from self to another deck/hand/discard pile
    pub fn give(&mut self, cards: Vec<Card>, other_deck: &mut Deck) {

    }
}

type Hand = Deck; // a hand of cards is really just a smaller deck
type DiscardPile = Deck; // A discard pile is just a faceup deck

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = false;
        for card in self.cards.iter() {
            if sep {
                write!(f, ", ")?;
            }
            write!(f, "{}", card)?;
            sep = true;
        }
        Ok(())
    }
}
