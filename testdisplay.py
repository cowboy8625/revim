#!/usr/bin/env python3

from collections import deque
from random import randint
from sys import stdout, stdin, exit
from os import system, name, O_NONBLOCK
from time import sleep
from termios import tcgetattr, ICANON, TCSANOW, ECHO, TCSAFLUSH, tcsetattr
from fcntl import fcntl, F_SETFL, F_GETFL
import os
import shlex
import struct
import platform
import subprocess


class Keys:
    ARROW_UP: str = "\x1b[A"
    ARROW_DOWN: str = "\x1b[B"
    ARROW_RIGHT: str = "\x1b[C"
    ARROW_LEFT: str = "\x1b[D"
    ESC: str = "\x1b"


def get_terminal_size():
    def ioctl_GWINSZ(fd):
        try:
            import fcntl
            import termios

            cr = struct.unpack("hh", fcntl.ioctl(fd, termios.TIOCGWINSZ, "1234"))
            return cr
        except:
            pass

    cr = ioctl_GWINSZ(0) or ioctl_GWINSZ(1) or ioctl_GWINSZ(2)
    if not cr:
        try:
            fd = os.open(os.ctermid(), os.O_RDONLY)
            cr = ioctl_GWINSZ(fd)
            os.close(fd)
        except:
            pass
    if not cr:
        try:
            cr = (os.environ["LINES"], os.environ["COLUMNS"])
        except:
            return None
    return int(cr[1]), int(cr[0])


WIDTH, HEIGHT = get_terminal_size()


def getchar(keys_len=5):
    fd = stdin.fileno()

    oldterm = tcgetattr(fd)
    newattr = tcgetattr(fd)
    newattr[3] = newattr[3] & ~ICANON & ~ECHO
    tcsetattr(fd, TCSANOW, newattr)

    oldflags = fcntl(fd, F_GETFL)
    fcntl(fd, F_SETFL, oldflags | O_NONBLOCK)

    try:
        while True:
            try:
                key = stdin.read(keys_len)
                break
            except IOError:
                pass
    finally:
        tcsetattr(fd, TCSAFLUSH, oldterm)
        fcntl(fd, F_SETFL, oldflags)
    return key


def hide():
    system("stty -echo")
    stdout.write("\033[?25l")
    stdout.flush()


def show():
    system("stty echo")
    stdout.write("\033[?25h")


def clear():
    system("cls" if name == "nt" else "clear")


def fg(r, g, b):
    return f"\x1b[38;2;{r};{g};{b}m"


def bg(r, g, b):
    return f"\x1b[48;2;{r};{g};{b}m"


def format_color(cell, fg=None, bg=None):
    if fg is not None and bg is not None:
        return f"{fg}{bg}{cell}\x1b[0m"
    elif fg is not None:
        return f"{fg}{cell}\x1b[0m"
    elif bg is not None:
        return f"{bg}{cell}\x1b[0m"
    else:
        return cell


class Pixel:
    def __init__(self, char, x, y, fg):
        self.char = char
        self.loc = Position(x, y)
        self.fg = fg

    def __str__(self):
        return f"{format_color(self.char, fg=self.fg)}"

    @classmethod
    def fromHeadFood(cls, head, food):
        return cls("#", head.loc.x, head.loc.y, food.fg)

    @property
    def x(self):
        return self.loc.x

    @property
    def y(self):
        return self.loc.y


class Position:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __iadd__(self, other):
        return Position((self.x + other.x), (self.y + other.y))

    def __eq__(self, other):
        return True if (self.x == other.x) and (self.y == other.y) else False


class Direction:
    UP = Position(0, -1)
    DOWN = Position(0, 1)
    LEFT = Position(-1, 0)
    RIGHT = Position(1, 0)


class Food:
    def __init__(self):
        self.spawn()

    def spawn(self):
        self.pixel = Pixel(
            "0",
            randint(0, WIDTH),
            randint(0, HEIGHT),
            fg(randint(0, 255), randint(0, 255), randint(0, 255)),
        )

    def draw(self):
        stdout.write(f"\x1b[{self.pixel.y};{self.pixel.x}H{self.pixel}")


class Snake:
    def __init__(self):
        self.head = Pixel("@", randint(0, WIDTH), randint(0, HEIGHT), fg(255, 0, 0))
        # Position(randint(0, WIDTH), randint(0, HEIGHT))
        self.direction = Direction.RIGHT
        self.length = 0
        self.body = []
        self.colors = []

    def change_direction(self, key):
        if key == Keys.ARROW_LEFT:
            self.direction = Direction.LEFT
        elif key == Keys.ARROW_RIGHT:
            self.direction = Direction.RIGHT
        elif key == Keys.ARROW_UP:
            self.direction = Direction.UP
        elif key == Keys.ARROW_DOWN:
            self.direction = Direction.DOWN

    def eat_food(self, food):
        if food.pixel.loc == self.head.loc:
            self.colors.insert(0, food.pixel.fg)
            food.spawn()
            self.length += 1
            return True
        else:
            return False

    def eat_self(self):
        for part in self.body:
            if part == self.head.loc:
                return True
        else:
            return False

    def update(self, food):
        if self.head.loc.x < 0:
            self.head.loc = Position(WIDTH, self.head.loc.y)
        elif self.head.loc.x > WIDTH:
            self.head.loc = Position(0, self.head.loc.y)
        elif self.head.loc.y < 0:
            self.head.loc = Position(self.head.loc.x, HEIGHT)
        elif self.head.loc.y > HEIGHT:
            self.head.loc = Position(self.head.loc.x, 0)
        self.body.insert(0, Pixel("#", self.head.x, self.head.y, food.pixel.fg))
        if self.length < len(self.body):
            self.body.pop()
        self.head.loc += self.direction

    def draw(self):
        stdout.write(f"\x1b[{self.head.y};{self.head.x}H{self.head}")
        for i, part in enumerate(self.body):
            part.fg = self.colors[i]
            stdout.write(f"\x1b[{part.y};{part.x}H{part}")


class Game:
    def __init__(self):
        self.snake = Snake()
        self.food = Food()
        self.tick_rate = 0.3

    def update(self):
        self.snake.update(self.food)
        if self.snake.eat_food(self.food):
            self.tick_rate -= 0.001
        if self.snake.eat_self():
            exit()

    def draw(self):
        self.snake.draw()
        self.food.draw()

    def mainloop(self):
        while True:
            key = getchar()
            clear()
            self.snake.change_direction(key)
            self.update()
            self.draw()
            stdout.flush()
            if key == Keys.ESC:
                break
            sleep(self.tick_rate)


def safe_run(func):
    hide()
    try:
        func()
    except Exception as e:
        print(e)
    finally:
        show()


def main():
    game = Game()
    game.mainloop()


if __name__ == "__main__":
    safe_run(main)
    # main()
