import game_of_war as gow

class GameManager:
    def __init__(self):
        self.Player1 = gow.Player("yungselm")
        self.Player2 = gow.Player("COM")
        self.deck = gow.Deck()
        self.game = gow.Game(self.Player1, self.Player2, self.deck)
        print(self.game)

    def play_game(self):
        # Initialize the game
        print("Initializing game...")
        self.game.initialize_game()
        
        # Play a round
        print("Playing first round...")
        outcome = self.game.play_round()
        print(f"Round outcome: {outcome}")
        
        # Continue playing rounds until the game is over
        while not self.game.game_over:
            print("Playing another round...")
            outcome = self.game.play_round()
            print(f"Round outcome: {outcome}")
        
        # Game over
        print(f"Game finished. Final outcome: {self.game.outcome}")

if __name__ == '__main__':
    manager = GameManager()
    manager.play_game()
