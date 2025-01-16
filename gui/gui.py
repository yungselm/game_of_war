from PyQt5.QtWidgets import QMainWindow
from PyQt5.QtGui import QPainter, QImage, QPen
from PyQt5.QtCore import Qt
from PIL import Image, ImageEnhance

import game_of_war as gow

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.graphical_deck = Image.open("media/full_set.png")
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
        self.show()

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
        # side = str(card.side).split(".")[1]
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
                    # self.pixelate_and_desaturate_image(card, image_path, 2)
                else:
                    image_path = "media/temp_card_p1.png"
                    card.save(image_path)
                    # self.pixelate_and_desaturate_image(card, image_path, 2)
            self.drawn_cards.append((x, y, image_path))
            self.update()

    @staticmethod   
    def pixelate_and_desaturate_image(image, output_image_path, pixel_size):
        width, height = image.size

        image_small = image.resize(
            (width // pixel_size, height // pixel_size),
            resample=Image.NEAREST
        )

        img_pixelated = image_small.resize(
            (width, height),
            Image.NEAREST
        )

        enhancer = ImageEnhance.Color(img_pixelated)
        img_desaturated = enhancer.enhance(0.5)

        enhancer = ImageEnhance.Brightness(img_desaturated)
        img_pastel = enhancer.enhance(1.2)

        blue_overlay = Image.new("RGB", img_pastel.size, (173, 216, 230))
        img_pastel = img_pastel.convert("RGB")
        blue_overlay = blue_overlay.convert("RGB")

        img_pastel = Image.blend(img_pastel, blue_overlay, 0.3)        

        img_pastel.save(output_image_path)

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
