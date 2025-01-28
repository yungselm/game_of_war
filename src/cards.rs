use pyo3::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl Value {
    fn rank(&self) -> u8 {
        match self {
            Value::Two => 0,
            Value::Three => 1,
            Value::Four => 2,
            Value::Five => 3,
            Value::Six => 4,
            Value::Seven => 5,
            Value::Eight => 6,
            Value::Nine => 7,
            Value::Ten => 8,
            Value::Jack => 9,
            Value::Queen => 10,
            Value::King => 11,
            Value::Ace => 12,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
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
        Card { suit, value, side }
    }

    pub fn flip(&mut self) {
        self.side = match self.side {
            Side::Front => Side::Back,
            Side::Back => Side::Front,
        };
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