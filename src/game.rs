use pyo3::prelude::*;
use super::cards::*;
use super::deck::*;
use super::player::Player;

#[derive(Debug, PartialEq, Eq, Clone)]
#[pyclass(eq, eq_int)]
pub enum Outcome {
    Player1Wins,
    Player2Wins,
    Tie,
    Running,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[pyclass]
pub struct Game {
    player1: Player,
    player2: Player,
    deck: Deck,
    table_cards: Vec<Card>,
    game_over: bool,
    outcome: Outcome,
}

#[pymethods]
impl Game {
    #[new]
    pub fn new(player1: Player, player2: Player, deck: Deck) -> Self {
        Game {
            player1,
            player2,
            deck,
            table_cards: Vec::new(),
            game_over: false,
            outcome: Outcome::Running,
        }
    }

    pub fn initialize_game(&mut self) {
        self.deck.shuffle();
        self.player1.initial_draw(&mut self.deck);
        self.player2.initial_draw(&mut self.deck);
    }

    pub fn play_round(&mut self) -> Outcome {
        let player1_card = self.player1.play_card();
        let player2_card = self.player2.play_card();

        self.table_cards.extend(
            vec![player1_card, player2_card]
                .into_iter()
                .filter_map(|card| card)
        );        
        for card in self.table_cards.iter_mut() {
            card.flip();
        }

        match (player1_card, player2_card) {
            (Some(player1_card), Some(player2_card)) => {
                if player1_card.value > player2_card.value {
                    self.player1.add_cards(self.table_cards.clone()); // why needed clone?
                    self.table_cards.clear();
                    self.evaluate_outcome()
                } else if player1_card.value < player2_card.value {
                    self.player2.add_cards(self.table_cards.clone());
                    self.table_cards.clear();
                    self.evaluate_outcome()
                } else {
                    // Go to War!!
                    println!("War!");
                    self.evaluate_outcome()
                }
            }
            (None, Some(_player2_card)) => {
                self.game_over = true;
                Outcome::Player2Wins
            }
            (Some(_player1_card), None) => {
                self.game_over = true;
                Outcome::Player1Wins
            }
            (None, None) => {
                self.game_over = true;
                Outcome::Tie
            }
        }
        
    }

    pub fn evaluate_outcome(&mut self) -> Outcome {
        if self.player1.get_player_deck().is_empty() {
            self.game_over = true;
            Outcome::Player2Wins
        } else if self.player2.get_player_deck().is_empty() {
            self.game_over = true;
            Outcome::Player1Wins
        } else if !self.player1.get_player_deck().is_empty() && !self.player2.get_player_deck().is_empty() {
            Outcome::Running
        } else {
            self.game_over = true;
            Outcome::Tie
        }
    }

    pub fn finish_game(&mut self) {
        while !self.game_over {
            self.play_round();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let player1 = Player::new("Player 1".to_string());
        let player2 = Player::new("Player 2".to_string());
        let deck = Deck::new();
        let game = Game::new(player1, player2, deck);
        assert_eq!(game.player1.get_player_name(), "Player 1");
        assert_eq!(game.player2.get_player_name(), "Player 2");
        assert_eq!(game.deck.get_deck().len(), 52);
        assert_eq!(game.table_cards.len(), 0);
        assert_eq!(game.game_over, false);
        assert_eq!(game.outcome, Outcome::Running);
    }

    #[test]
    fn test_initialize_game() {
        let mut player1 = Player::new("Player 1".to_string());
        let mut player2 = Player::new("Player 2".to_string());
        let mut deck = Deck::new();
        let mut game = Game::new(player1, player2, deck);
        game.initialize_game();
        assert_eq!(game.player1.get_player_deck().len(), 26);
        assert_eq!(game.player2.get_player_deck().len(), 26);
    }

    #[test]
    fn test_play_round() {
        let mut player1 = Player::new("Player 1".to_string());
        let mut player2 = Player::new("Player 2".to_string());
        let mut deck = Deck::new();
        let mut game = Game::new(player1, player2, deck);
        game.initialize_game();
        let outcome = game.play_round();
        assert_eq!(game.table_cards.len(), 2);
        assert_eq!(outcome, Outcome::Running);
    }

    #[test]
    fn test_evaluate_outcome() {
        let mut player1 = Player::new("Player 1".to_string());
        let mut player2 = Player::new("Player 2".to_string());
        let mut deck = Deck::new();
        let mut game = Game::new(player1, player2, deck);
        game.initialize_game();
        let outcome = game.evaluate_outcome();
        assert_eq!(outcome, Outcome::Running);
    }

    #[test]
    fn test_finish_game() {
        let mut player1 = Player::new("Player 1".to_string());
        let mut player2 = Player::new("Player 2".to_string());
        let mut deck = Deck::new();
        let mut game = Game::new(player1, player2, deck);
        game.initialize_game();
        game.finish_game();
        assert_eq!(game.game_over, true);
    }
}