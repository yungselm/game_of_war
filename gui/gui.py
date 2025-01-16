from PyQt5 import QtCore, QtWidgets, QtGui
from PyQt5.QtWidgets import QMainWindow, QLabel, QPushButton
from PyQt5.QtCore import Qt
from PyQt5.QtGui import QPainter, QImage, QPen
from PIL import Image
import game_of_war as gow

from .game_manager import GameManager

class Layer(QtCore.QObject):
    def __init__(self, host, child, alignment=Qt.AlignLeft, setWidth=False, setHeight=False, parent=None):
        super().__init__(parent)
        self._host = host
        self._child = child
        self._alignment = alignment
        self._setWidth = setWidth
        self._setHeight = setHeight
        child.setParent(host)
        host.installEventFilter(self)

    def eventFilter(self, watched, event):
        if watched != self._host or event.type() != QtCore.QEvent.Resize:
            return False
        hostSize = event.size()
        childSize = self._child.sizeHint()
        alignment = self._alignment
        x = 0
        y = 0
        dWidth = max(0, hostSize.width() - childSize.width())
        dHeight = max(0, hostSize.height() - childSize.height())
        if alignment & Qt.AlignRight:
            x = dWidth
        elif alignment & Qt.AlignHCenter:
            x = dWidth / 2
        if alignment & Qt.AlignVCenter:
            y = dHeight / 2
        elif alignment & Qt.AlignBottom:
            y = dHeight
        width = hostSize.width() if self._setWidth else childSize.width()
        height = hostSize.height() if self._setHeight else childSize.height()
        self._child.setGeometry(x, y, width, height)
        return False

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.graphical_deck = Image.open("media/full_set.png")
        self.game_manager = GameManager()

        # only needed for testing
        self.Player1 = gow.Player("yungselm")
        self.Player2 = gow.Player("COM")
        self.deck = gow.Deck()
        self.game = gow.Game(self.Player1, self.Player2, self.deck)

        self.drawn_cards = []
        self.x_player1_card, self.y_player1_card = 750, 500
        self.x_player2_card, self.y_player2_card = 750, 150
        self.x_player1_war_card, self.y_player1_war_card = 1000, 500
        self.x_player2_war_card, self.y_player2_war_card = 1000, 150
        self.x_player1_deck, self.y_player1_deck = 80, 600
        self.x_player2_deck, self.y_player2_deck = 1600, 60

        self.init_gui()

    def __call__(self):
        # Test: draw a card
        self.deck.shuffle()
        card1 = self.deck.draw()
        print("card1: ", card1)
        self.draw_card(self.Player1, card1)
        card2 = self.deck.draw()
        print("card2: ", card2)
        self.draw_card(self.Player2, card2)

    def init_gui(self):
        self.setWindowTitle("Game of War")
        self.setStyleSheet("background-color: green;")
        screen = self.screen().availableGeometry()
        self.setGeometry(screen)

        # Add buttons at the specified positions and sizes
        self.add_buttons()

        self.show()

    def add_buttons(self):
        # Create two buttons
        button1 = QPushButton("Start Game", self)
        button1.setGeometry(750, 900, 200, 50)
        button1.setStyleSheet("background-color: white; color: green; font-size: 20px;")
        button1.clicked.connect(self.start_game)  # Placeholder for start game method

        button2 = QPushButton("Play Round", self)
        button2.setGeometry(1000, 900, 200, 50)
        button2.setStyleSheet("background-color: white; color: green; font-size: 20px;")
        button2.clicked.connect(self.play_round)  # Placeholder for reset game method

        # Use the Layer class to manage widget positioning
        Layer(self, button1, Qt.AlignLeft)
        Layer(self, button2, Qt.AlignLeft)

    def start_game(self):
        GameManager.play_game(self)
        print("Game started")

    def play_round(self):
        GameManager.play_game(self)
        print("Play round")

    def draw_card(self, player, card):
        hashmap_cropping = {
            "Hearts": [42, 228],
            "Diamonds": [238, 425],
            "Spades": [434, 621],
            "Clubs": [630, 817],
            "King": [35, 167],
            "Queen": [177, 309],
            "Jack": [321, 453],
            "Ten": [464, 596],
            "Nine": [607, 739],
            "Eight": [750, 882],
            "Seven": [893, 1026],
            "Six": [1036, 1170],
            "Five": [1179, 1312],
            "Four": [1322, 1455],
            "Three": [1465, 1598],
            "Two": [1608, 1741],
            "Ace": [1751, 1884]
        }

        value = str(card.value).split(".")[1]
        suit = str(card.suit).split(".")[1]
        side = "Front"
        print(f"Card: {value} of {suit}, Side: {side}")

        y1, y2 = hashmap_cropping[suit]
        x1, x2 = hashmap_cropping[value]
        print("coordinates are: ", x1, y1, x2, y2)

        if card is not None: 
            x, y = (self.x_player1_card, self.y_player1_card) if player == self.Player1 else (self.x_player2_card, self.y_player2_card)
            if side == "Back":
                image_path = "media/card_back.png"
            else:
                card = self.graphical_deck.crop((x1, y1, x2, y2))
                card = card.resize((200, 300))
                if player == self.Player2:
                    image_path = "media/temp_card_p2.png"
                    card.save(image_path)
                else:
                    image_path = "media/temp_card_p1.png"
                    card.save(image_path)
            self.drawn_cards.append((x, y, image_path))
            self.update()

    def paintEvent(self, event):
        painter = QPainter(self)
        
        # Draw the scene
        painter.setPen(QPen(Qt.white, 5, Qt.DotLine))
        painter.drawRect(1600, 60, 200, 300)
        painter.drawRect(80, 600, 200, 300)
        painter.drawRect(750, 500, 200, 300)
        painter.drawRect(1000, 500, 200, 300)
        painter.drawRect(750, 150, 200, 300)
        painter.drawRect(1000, 150, 200, 300)

        # Draw cards
        for x, y, image_path in self.drawn_cards:
            card_image = QImage(image_path)
            painter.drawImage(x, y, card_image)
        
        painter.end()
