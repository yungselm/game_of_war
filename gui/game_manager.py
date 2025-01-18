import game_of_war as gow

from PyQt5 import QtCore
from PyQt5.QtCore import QObject

class GameManager(QObject):
    player_updated = QtCore.pyqtSignal()  # Emit when state changes
    deck_updated = QtCore.pyqtSignal()
    game_updated = QtCore.pyqtSignal()

    def __init__(self):
        super().__init__()
        self.Player1 = gow.Player("yungselm")
        self.Player2 = gow.Player("COM")
        self.deck = gow.Deck()
        self.game = gow.Game(self.Player1, self.Player2, self.deck)
        self.outcome = None
        print("Initial Game state:", self.game)

    def initialize_game(self):
        print("Initializing game...")
        self.game.initialize_game()
        self.Player1 = self.game.player1
        self.Player2 = self.game.player2
        self.player_updated.emit()  # Notify GUI of updated player states
        self.deck_updated.emit()    # Notify GUI of deck updates
        self.game_updated.emit()    # Notify GUI of game state updates

    def play_round(self):
        print("Playing round...")
        print("Game currently:", self.game)
        self.game.play_round()
        self.Player1 = self.game.player1
        self.Player2 = self.game.player2
        print("Game after move:", self.game)
        outcome = self.game.outcome
        print(f"Round outcome: {outcome}")
        self.player_updated.emit()
        self.deck_updated.emit()
        self.game_updated.emit()

    def play_game(self):
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
        print(self.game)

if __name__ == '__main__':
    manager = GameManager()
    manager.play_game()
