import loguru as logger

import game_of_war as gow
from PyQt5.QtGui import QImage
from PIL import Image, ImageDraw, ImageFont, ImageQt

import matplotlib.pyplot as plt

# media/full_set.png contains a image with 52 cards, I want to define a hashmap to map the card to its position inside of the image
image = Image.open("media/full_set.png")
width, height = image.size
card_width, card_height = 200, 300

plt.imshow(image)
plt.show()
image = Image.open("media/full_set.png")
card = image.crop((35, 42, 168, 228))

# first card: start y = 42, x= 35, end y=228, x=168
# reload image with only these x and y coordinates
# second row, first card:  y = 238, x = 35, y= 425, x=168
# third row, first card: y = 434, x = 35, y = 621, x = 168
# fourth row, first card: y = 630, x = 35, y = 817, x = 168

# first row, second card: x = 178, y = 42, x = 311, y = 228
# first row, third card: x = 321, y = 42, x = 454, y = 228
# first row, fourth card: x = 467, y = 42, x = 600, y = 228
# first row, fifth card: x = 612, y = 42, x = 745, y = 228
# first row, sixth card: x = 756, y = 42, x = 889, y = 228
# first row, seventh card: x = 900, y = 42, x = 1033, y = 228
# first row, eighth card: x = 1044, y = 42, x = 1177, y = 228
# first row, ninth card: x = 1188, y = 42, x = 1321, y = 228
# first row, tenth card: x = 1332, y = 42, x = 1465, y = 228
# first row, eleventh card: x = 1476, y = 42, x = 1609, y = 228
# first row, twelfth card: x = 1620, y = 42, x = 1753, y = 228
# first row, thirteenth card: x = 1764, y = 42, x = 1897, y = 228

# row order: heart, diamond, spade, club
# column order: king, queen, jack, 10, 9, 8, 7, 6, 5, 4, 3, 2, ace

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
    "Three": [1476, 1609],
    "Two": [1608, 1741],
    "Ace": [1751, 1884]
}

value = "Ace"
suit = "Clubs"

# Get the cropping coordinates
y1, y2 = hashmap_cropping[suit]
x1, x2 = hashmap_cropping[value]

# Crop the card
card = image.crop((x1, y1, x2, y2))
plt.imshow(card)
plt.show()