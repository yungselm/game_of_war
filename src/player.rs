use pyo3::prelude::*;
use super::cards::*;
use super::deck::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[pyclass(eq, eq_int)]
pub enum PlayerState {
    Alive,
    Dead,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[pyclass]
pub struct Player {
    player_name: String,
    player_deck: Vec<Card>,
    dead_or_alive: PlayerState,
}

#[pymethods]
impl Player {
    #[new]
    pub fn new(player_name: String) -> Self {
        Player {
            player_name,
            player_deck: Vec::new(),
            dead_or_alive: PlayerState::Alive,
        }
    }

    pub fn initial_draw(&mut self, deck: &mut Deck) {
        for _ in 0..26 {
            self.player_deck.push(deck.draw().unwrap());
        }
    }

    pub fn play_card(&mut self, face_up: bool) -> Option<Card> {
        let card = self.player_deck.pop();
        let card = match card {
            Some(mut card) => {
                if face_up {
                    card.side = Side::Front;
                }
                card
            },
            _ => return None,
        };
        if self.player_deck.is_empty() {
            self.dead_or_alive = PlayerState::Dead;
        }
        Some(card)
    }

    pub fn add_cards(&mut self, mut cards: Vec<Card>) {
        // ensure that all cards have "Back" when adding to player_deck
        for card in cards.iter_mut() {
            if card.side == Side::Front {
                card.side = Side::Back;
            }
        }

        let card = self.player_deck.extend(cards);
        if !self.player_deck.is_empty() {
            self.dead_or_alive = PlayerState::Alive;
        }
        card
    }

    pub fn get_player_name(&self) -> String {
        self.player_name.clone()
    }

    pub fn get_player_deck(&self) -> Vec<Card> {
        self.player_deck.clone()
    }

    // needed for Python to print the object
    pub fn __repr__(&self) -> String {
        format!("Player: {}, Deck: {:?}, Status: {:?}", 
        self.player_name, 
        self.player_deck, 
        self.dead_or_alive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_player() {
        let test_player = Player::new("Test Player".to_string());
        assert_eq!(test_player.player_name, "Test Player");
        assert_eq!(test_player.player_deck.len(), 0);
        assert_eq!(test_player.dead_or_alive, PlayerState::Alive);
    }

    #[test]
    fn test_initial_draw() {
        let mut test_player = Player::new("Test Player".to_string());
        let mut test_deck = Deck::new();
        test_player.initial_draw(&mut test_deck);
        assert_eq!(test_player.player_deck.len(), 26);
    }

    #[test]
    fn test_play_card() {
        let mut test_player = Player::new("Test Player".to_string());
        let mut test_deck = Deck::new();
        test_player.initial_draw(&mut test_deck);
        let card = test_player.play_card(true);
        assert_eq!(test_player.player_deck.len(), 25);
        assert_eq!(card.is_some(), true);
    }

    #[test]
    fn test_add_cards() {
        let mut test_player = Player::new("Test Player".to_string());
        let mut test_deck = Deck::new();
        test_player.initial_draw(&mut test_deck);
        let mut test_player2 = Player::new("Test Player 2".to_string());
        test_player2.initial_draw(&mut test_deck);
        let cards = test_player2.player_deck.clone();
        test_player.add_cards(cards);
        assert_eq!(test_player.player_deck.len(), 52);
    }

    #[test]
    // dies after playing last card
    fn test_dead_or_alive() {
        let mut test_player = Player::new("Test Player".to_string());
        let mut test_deck = Deck::new();
        test_player.initial_draw(&mut test_deck);
        assert_eq!(test_player.dead_or_alive, PlayerState::Alive);
        for _ in 0..26 {
            test_player.play_card(true);
        }
        assert_eq!(test_player.dead_or_alive, PlayerState::Dead);
    }

    #[test]
    // revived after adding cards
    fn test_dead_or_alive_revived() {
        let mut test_player1 = Player::new("Test Player 1".to_string());
        let mut test_deck = Deck::new();
        test_player1.initial_draw(&mut test_deck);
        for _ in 0..25 {
            test_player1.play_card(true);
        }
        test_player1.play_card(true);
        println!("{:?}", test_player1.dead_or_alive);
        assert_eq!(test_player1.dead_or_alive, PlayerState::Dead);
        let cards = vec![Card::new(Suit::Hearts, Value::Ace, Side::Back), Card::new(Suit::Hearts, Value::Two, Side::Back)];
        test_player1.add_cards(cards);
        println!("{:?}", test_player1.dead_or_alive);
        assert_eq!(test_player1.dead_or_alive, PlayerState::Alive);
    }
}