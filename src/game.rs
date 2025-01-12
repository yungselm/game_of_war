use pyo3::prelude::*;
// use super::cards::*;
use super::deck::*;
use super::player::*;

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
    game_over: bool,
    outcome: Option<String>,
}

#[pymethods]
impl Game {
    fn new(player1: Player, player2: Player, deck: Deck) -> Self {
        Game {
            player1,
            player2,
            deck,
            game_over: false,
            outcome: None,
        }
    }

    fn play(&mut self) -> Outcome {
        while !self.game_over {
            let player1_card = self.deck.draw().unwrap();
            let player2_card = self.deck.draw().unwrap();
            if player1_card.value > player2_card.value {
                self.player1.add_card(player1_card);
                self.player1.add_card(player2_card);
            } else if player1_card.value < player2_card.value {
                self.player2.add_card(player1_card);
                self.player2.add_card(player2_card);
            } else {
                // war
                let mut war_cards = vec![player1_card, player2_card];
                let mut war_over = false;
                while !war_over {
                    let player1_war_card = self.deck.draw().unwrap();
                    let player2_war_card = self.deck.draw().unwrap();
                    war_cards.push(player1_war_card);
                    war_cards.push(player2_war_card);
                    if player1_war_card.value > player2_war_card.value {
                        self.player1.add_cards(war_cards);
                        war_over = true;
                    } else if player1_war_card.value < player2_war_card.value {
                        self.player2.add_cards(war_cards);
                        war_over = true;
                    }
                }
            }
            if self.player1.get_hand().len() == 0 {
                self.game_over = true;
                return Outcome::Player2Wins;
            } else if self.player2.get_hand().len() == 0 {
                self.game_over = true;
                return Outcome::Player1Wins;
            }
        }
    }
}