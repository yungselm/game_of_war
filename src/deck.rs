use pyo3::prelude::*;
use super::cards::{Card, Suit, Value, Side};
use rand::seq::SliceRandom;

#[pyclass]
pub struct Deck {
    deck: Vec<Card>,
}

#[pymethods]
impl Deck {
    #[new]
    fn new() -> Self {
        let mut deck = Vec::new();
        for &suit in &[Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for &value in &[Value::Two, Value::Three, Value::Four, Value::Five, Value::Six, Value::Seven, Value::Eight, Value::Nine, Value::Ten, Value::Jack, Value::Queen, Value::King, Value::Ace] {
                deck.push(Card::new(suit, value, Side::Back));
            }
        }
        Deck { deck }
    }

    fn shuffle(&mut self) {
        self.deck.shuffle(&mut rand::thread_rng());
    }

    fn draw(&mut self) -> Option<Card> {
        self.deck.pop() // only removes one value, now players have to repeatedly draw cards, even though need half the deck --> fix later
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