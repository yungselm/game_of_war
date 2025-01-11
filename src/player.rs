mod player {
    use super::cards::{Card, Suit, Value, Side};
    use super::deck::Deck;

    pub struct Player {
        player_deck: PlayerDeck,
    }

    impl Player {
        pub fn new() -> Player {
            Player { hand: VecDeque::new() }
        }

        pub fn draw(&mut self, card: Card) {
            self.hand.push_back(card);
        }

        pub fn play(&mut self) -> Option<Card> {
            self.hand.pop_front()
        }
    }
}