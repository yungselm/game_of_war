use pyo3::prelude::*;
use super::cards::{Card, Suit, Value, Side};
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Eq, Clone)]
#[pyclass]
pub struct Deck {
    deck: Vec<Card>,
}

#[pymethods]
impl Deck {
    #[new]
    pub fn new() -> Self {
        let deck = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs]
            .iter()
            .flat_map(|&suit| {
                [
                    Value::Two, Value::Three, Value::Four, Value::Five, Value::Six, Value::Seven,
                    Value::Eight, Value::Nine, Value::Ten, Value::Jack, Value::Queen, Value::King, Value::Ace,
                ]
                .iter()
                .map(move |&value| Card::new(suit, value, Side::Back))
            })
            .collect();
        Deck { deck }
    }

    pub fn shuffle(&mut self) {
        self.deck.shuffle(&mut rand::thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.deck.pop() // only removes one value, now players have to repeatedly draw cards, even though need half the deck --> fix later
    }

    // needed for python print (debugging)
    pub fn len(&self) -> usize {
        self.deck.len()
    }

    // needed for testing
    pub fn push(&mut self, card: Card) {
        self.deck.push(card);
    }

    // needed for testing
    pub fn get_deck(&self) -> Vec<Card> {
        self.deck.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck() {
        let test_deck = Deck::new();
        assert_eq!(test_deck.deck.len(), 52);
    }

    #[test]
    fn test_shuffle_deck() {
        let mut test_deck = Deck::new();
        let original_cards = test_deck.deck.clone();
        test_deck.shuffle();
        assert_ne!(test_deck.deck, original_cards);
    }

    #[test]
    fn test_draw_card() {
        let mut test_deck = Deck::new();
        let card = test_deck.draw();
        assert_eq!(test_deck.deck.len(), 51);
        assert_eq!(card.is_some(), true);
    }
}