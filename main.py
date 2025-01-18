import sys

from PyQt5.QtWidgets import QApplication
from gui.gui import MainWindow

def main():
    app = QApplication(sys.argv)

    window = MainWindow()
    window.show()

    window()

    app.aboutToQuit.connect(window.cleanup_temp_files)

    sys.exit(app.exec_())

if __name__ == '__main__':
    main()
