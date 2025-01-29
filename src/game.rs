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
    ItsWar, // only needed so python can catch the intermediate stage, to draw a coupled event
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
    last_round_winner: Option<Player>,
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
            last_round_winner: None,
        }
    }

    pub fn initialize_game(&mut self) {
        self.deck.shuffle();
        self.player1.initial_draw(&mut self.deck);
        self.player2.initial_draw(&mut self.deck);
    }

    pub fn play_round(&mut self) -> Outcome {
        let player1_card = self.player1.play_card(true);
        let player2_card = self.player2.play_card(true);

        self.table_cards.extend(
            vec![player1_card, player2_card]
                .into_iter()
                .filter_map(|card| card),
        );

        match (player1_card, player2_card) {
            (Some(player1_card), Some(player2_card)) => {
                if player1_card.value > player2_card.value {
                    self.player1.add_cards(self.table_cards.clone());
                    self.table_cards.clear();
                    self.last_round_winner = Some(self.player1.clone());
                    self.evaluate_outcome()
                } else if player1_card.value < player2_card.value {
                    self.player2.add_cards(self.table_cards.clone());
                    self.table_cards.clear();
                    self.last_round_winner = Some(self.player2.clone());
                    self.evaluate_outcome()
                } else {
                    println!("War!");
                    self.handle_war()
                }
            }
            (None, Some(_)) => {
                self.game_over = true;
                Outcome::Player2Wins
            }
            (Some(_), None) => {
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
        match (
            self.player1.get_player_deck().is_empty(),
            self.player2.get_player_deck().is_empty(),
        ) {
            (true, true) => {
                self.game_over = true;
                Outcome::Tie
            }
            (true, false) => {
                self.game_over = true;
                Outcome::Player2Wins
            }
            (false, true) => {
                self.game_over = true;
                Outcome::Player1Wins
            }
            (false, false) => Outcome::Running,
        }
    }

    fn handle_war(&mut self) -> Outcome {
        self.outcome = Outcome::ItsWar;

        let player_1_facedown = self.player1.play_card(false);
        let player_2_facedown = self.player2.play_card(false);

        let player_1_faceup = self.player1.play_card(true);
        let player_2_faceup = self.player2.play_card(true);

        self.table_cards.extend(
            vec![player_1_facedown, player_2_facedown, player_1_faceup, player_2_faceup]
                .into_iter()
                .filter_map(|card| card),
        );

        match (player_1_faceup, player_2_faceup) {
            (Some(player1_card), Some(player2_card)) => {
                if player1_card.value > player2_card.value {
                    self.player1.add_cards(self.table_cards.clone());
                    self.table_cards.clear();
                    self.last_round_winner = Some(self.player1.clone());
                    self.evaluate_outcome()
                } else if player1_card.value < player2_card.value {
                    self.player2.add_cards(self.table_cards.clone());
                    self.table_cards.clear();
                    self.last_round_winner = Some(self.player2.clone());
                    self.evaluate_outcome()
                } else {
                    println!("War continues!");
                    self.handle_war()
                }
            }
            (None, Some(_)) => {
                self.game_over = true;
                Outcome::Player2Wins
            }
            (Some(_), None) => {
                self.game_over = true;
                Outcome::Player1Wins
            }
            (None, None) => {
                self.game_over = true;
                Outcome::Tie
            }
        }
    }

    // mainly used for testing and debugging
    pub fn finish_game(&mut self) {
        while !self.game_over {
            self.play_round();
        }
    }

    // needed for Python to print the object
    pub fn __repr__(&self) -> String {
        format!(
            "Game(player1: {:?}, player2: {:?}, deck size: {}, table cards: {:?}, outcome: {:?})",
            self.player1, self.player2, self.deck.len(), self.table_cards, self.outcome
        )
    }

    // tried with references, but python needs clone because of lifetimes in Rust
    #[getter]
    pub fn player1(&self) -> Player {
        self.player1.clone()
    }

    #[getter]
    pub fn player2(&self) -> Player {
        self.player2.clone()
    }

    #[getter]
    pub fn deck(&self) -> Deck {
        self.deck.clone()
    }

    #[getter]
    pub fn get_table_cards(&self) -> Vec<Card> {
        self.table_cards.clone()
    }

    pub fn get_last_played_cards(&self) -> (Option<Card>, Option<Card>) {
        (
            self.player1.get_last_played_card(),
            self.player2.get_last_played_card(),
        )
    }

    #[getter]
    pub fn outcome(&self) -> Outcome {
        self.outcome.clone()
    }

    #[getter]
    pub fn last_round_winner(&self) -> Option<Player> {
        self.last_round_winner.clone()
    }

    #[getter]
    pub fn game_over(&self) -> bool {
        self.game_over
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
        let player1 = Player::new("Player 1".to_string());
        let player2 = Player::new("Player 2".to_string());
        let deck = Deck::new();
        let mut game = Game::new(player1, player2, deck);
        game.initialize_game();
        assert_eq!(game.player1.get_player_deck().len(), 26);
        assert_eq!(game.player2.get_player_deck().len(), 26);
    }

    #[test]
    fn test_play_round() {
        let player1 = Player::new("Player 1".to_string());
        let player2 = Player::new("Player 2".to_string());
        let deck = Deck::new();
        let mut game = Game::new(player1, player2, deck);
        game.initialize_game();
        let outcome = game.play_round();
        assert_eq!(game.table_cards.len(), 0);
        assert_ne!(game.player1.get_player_deck().len(), 26);
        assert_ne!(game.player2.get_player_deck().len(), 26);
        assert_eq!(outcome, Outcome::Running);
    }

    #[test]
    fn test_evaluate_outcome() {
        let player1 = Player::new("Player 1".to_string());
        let player2 = Player::new("Player 2".to_string());
        let deck = Deck::new();
        let mut game = Game::new(player1, player2, deck);
        game.initialize_game();
        let outcome = game.evaluate_outcome();
        assert_eq!(outcome, Outcome::Running);
    }

    #[test]
    fn test_finish_game() {
        let player1 = Player::new("Player 1".to_string());
        let player2 = Player::new("Player 2".to_string());
        let deck = Deck::new();
        let mut game = Game::new(player1, player2, deck);
        game.initialize_game();
        game.finish_game();
        assert_eq!(game.game_over, true);
    }
}