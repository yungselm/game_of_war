from loguru import logger

from PyQt5.QtWidgets import QMainWindow
from PyQt5.QtGui import QPen, QPainter, QImage
from PyQt5.QtCore import Qt

import game_of_war as gow
from .dynamic_card_generator import *

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Game of War")
        screen = self.screen().availableGeometry()
        self.setGeometry(screen)
        self.draw_scene()
        self.paintEvent(screen)

        self.deck = gow.Deck()

        self.show()

    def draw_scene(self):
        logger.debug("Drawing scene...") 
        # green background
        self.setStyleSheet("background-color: green;")

    def paintEvent(self, event):
        painter = QPainter(self)
        painter.setPen(QPen(Qt.white, 5, Qt.DotLine))
        
        # player decks
        painter.drawRect(1600, 60, 200, 300)
        painter.drawRect(80, 600, 200, 300)

        # table decks
        painter.drawRect(750, 500, 200, 300)
        painter.drawRect(1000, 500, 200, 300)
        painter.drawRect(750, 150, 200, 300)
        painter.drawRect(1000, 150, 200, 300)

        # display a .jpeg image on the screen
        image = QImage('media/card_back.png')
        painter.drawImage(1600, 60, image)

        image2 = create_card_image("Two", "Clubs", "Front")
        painter.drawImage(750, 150, image2)