use pyo3::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)] // Add the Clone and Copy traits to the Suit enum, otherwise conversion to python objects fails
#[pyclass(eq, eq_int)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[pyclass(eq, eq_int)]
pub enum Value {
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

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Value::*;
        let self_rank = match self{
            Two => 0,
            Three => 1,
            Four => 2,
            Five => 3,
            Six => 4,
            Seven => 5,
            Eight => 6,
            Nine => 7,
            Ten => 8,
            Jack => 9,
            Queen => 10,
            King => 11,
            Ace => 12,
        };
        let other_rank = match other {
            Two => 0,
            Three => 1,
            Four => 2,
            Five => 3,
            Six => 4,
            Seven => 5,
            Eight => 6,
            Nine => 7,
            Ten => 8,
            Jack => 9,
            Queen => 10,
            King => 11,
            Ace => 12,
        };
        self_rank.cmp(&other_rank)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[pyclass(eq, eq_int)]
pub enum Side {
    Front,
    Back,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[pyclass]
pub struct Card {
    #[pyo3(get, set)]
    pub suit: Suit,
    #[pyo3(get, set)]
    pub value: Value,
    #[pyo3(get, set)]
    pub side: Side,
}

#[pymethods]
impl Card {
    #[new]
    pub fn new(suit: Suit, value: Value, side: Side) -> Self {
        Card { suit, value, side}
    }

    pub fn flip(&mut self) {
        self.side = match self.side {
            Side::Front => Side::Back,
            Side::Back => Side::Front,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_card() {
        let card = Card::new(Suit::Spades, Value::Ace, Side::Back);
        assert_eq!(card.suit, Suit::Spades);
        assert_eq!(card.value, Value::Ace);
        assert_eq!(card.side, Side::Back);
    }
}