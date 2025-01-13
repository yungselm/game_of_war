import loguru as logger

import game_of_war as gow
from PyQt5.QtGui import QImage
from PIL import Image, ImageDraw, ImageFont, ImageQt

import matplotlib.pyplot as plt

def hashmap_font(value):
    rank_map = {
        "Two": "2", "Three": "3", "Four": "4", "Five": "5", 
        "Six": "6", "Seven": "7", "Eight": "8", "Nine": "9",
        "Ten": "10", "Jack": "J", "Queen": "Q", "King": "K", "Ace": "A"
    }
    return rank_map.get(value, value)  # Default to original value if not in map

def create_card_image(value, suit, side, output_file="card.png"):
    width, height = 200, 300  # Card dimensions
    if side == "Back":
        # Load the back image
        image = Image.open("media/card_back.png")
    else:
        # Create a blank card
        card_color = "white"
        image = Image.new("RGB", (width, height), card_color)
        draw = ImageDraw.Draw(image)

        # Load the suit icon
        suit_image_path = f"media/{suit.lower()}.png"
        suit_image = Image.open(suit_image_path).resize((50, 50))

        # Define fonts
        try:
            font = ImageFont.truetype("arial.ttf", 30)
        except IOError:
            font = ImageFont.load_default(size=30)

        # Get rank symbol
        rank = hashmap_font(value)

        if suit in ["Spades", "Clubs"]:
            draw.text((10, 5), rank, fill="black", font=font)
            draw.text((170, 260), rank, fill = "black", font=font)
        else:
            draw.text((10, 5), rank, fill = "red", font=font)
            draw.text((170, 260), rank, fill = "red", font=font)

        image.paste(suit_image, (75, 130), suit_image)
    
    return image

value = "Ace"
suit = "Hearts"
side = "Front"
card_image = create_card_image(value, suit, side)

# Display the card image
plt.imshow(card_image)
plt.axis("off")
plt.show()

deck = gow.Deck()
deck.shuffle()
drawn_card = deck.draw()

value = str(drawn_card.value).split(".")[1]
suit = str(drawn_card.suit).split(".")[1]
side = "Front"

card_image = create_card_image(value, suit, side)

# Display the card image
plt.imshow(card_image)
plt.axis("off")
plt.show()
