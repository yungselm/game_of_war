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
            outcome: None,
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

        self.table_cards.extend(vec![player1_card, player2_card]);

        match (player1_card, player2_card) {
            (Some(player1_card), Some(player2_card)) => {
                if player1_card.value > player2_card.value {
                    self.player1.add_cards(self.table_cards);
                    self.table_cards.clear();
                    self.evaluate_outcome(player1, player2)
                } else if player1_card.value < player2_card.value {
                    self.player2.add_cards(self.table_cards);
                    self.table_cards.clear();
                } else {
                    // Go to War!!
                }
            }
            (None, Some(player2_card)) => {
                self.outcome = Some("Player 2 wins!".to_string());
                self.game_over = true;
                Outcome::Player2Wins
            }
            (Some(player1_card), None) => {
                self.outcome = Some("Player 1 wins!".to_string());
                self.game_over = true;
                Outcome::Player1Wins
            }
            (None, None) => {
                self.outcome = Some("Tie!".to_string());
                self.game_over = true;
                Outcome::Tie
            }
        }
        
    }

    pub fn evaluate_outcome(&self, player1: Player, player2: Player) -> Option<String> {
        if player1.get_player_deck().is_empty() {
            Outcome::Player2Wins
        } else if player2.get_player_deck().is_empty() {
            Outcome::Player1Wins
        } else {
            None
        }
    }

    pub fn finish_game(&mut self) {
    }
}