
from typing import Callable
import argparse
import itertools

class bcolors:
    INVERT = '\033[7m'
    RST_INVERT = '\033[27m'

class Piece:
    def __init__(self, m_vects, d_vects, char, is_white):
        self.m_vects = m_vects
        self.d_vects = d_vects
        self.char = char
        self.is_white = is_white


#board = [
#    ["r","p"," "," "," "," ","P","R"],
#    ["n","p"," "," "," "," ","P","N"],
#    ["b","p"," "," "," "," ","P","B"],
#    ["q","p"," "," "," "," ","P","Q"],
#    ["k","p"," "," "," "," ","P","K"],
#    ["b","p"," "," "," "," ","P","B"],
#    ["n","p"," "," "," "," ","P","N"],
#    ["r","p"," "," "," "," ","P","R"]
#]

# m vectors: exhaustive list of (m)oves with positions relative to current position
# d vectors: move (d)irections for use in a loop

wpawn_mvects = [(0, 0), (-1, -1), (0, -1), (1, -1)]
bpawn_mvects = [(0, 0), (-1, 1), (0, 1), (1, 1)]

rook_dvects = [(1, 0), (0, 1)]
knight_mvects = [(0, 0), (-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)]
bishop_dvects = [(1, 1), (1, -1)]

king_mvects= [(-1, -1), (0, -1), (1, -1),
               (-1,  0), (0,  0), (1,  0),
               (-1,  1), (0,  1), (1,  1)]

queen_dvects = rook_dvects[:]
queen_dvects.extend(bishop_dvects[:])

wpawn = Piece(wpawn_mvects, None, "P", True)
bpawn = Piece(bpawn_mvects, None, "p", False)

board = [
    [Piece(None, rook_dvects,   "r", False), bpawn, None, None, None, None, wpawn, Piece(None, rook_dvects,   "R", True)],
    [Piece(knight_mvects, None, "n", False), bpawn, None, None, None, None, wpawn, Piece(knight_mvects, None, "N", True)],
    [Piece(None, bishop_dvects, "b", False), bpawn, None, None, None, None, wpawn, Piece(None, bishop_dvects, "B", True)],
    [Piece(None, queen_dvects,  "q", False), bpawn, None, None, None, None, wpawn, Piece(None, queen_dvects,  "Q", True)],
    [Piece(king_mvects, None,   "k", False), bpawn, None, None, None, None, wpawn, Piece(king_mvects, None,   "K", True)],
    [Piece(None, bishop_dvects, "b", False), bpawn, None, None, None, None, wpawn, Piece(None, bishop_dvects, "B", True)],
    [Piece(knight_mvects, None, "n", False), bpawn, None, None, None, None, wpawn, Piece(knight_mvects, None, "N", True)],
    [Piece(None, rook_dvects,   "r", False), bpawn, None, None, None, None, wpawn, Piece(None, rook_dvects,   "R", True)],
]


def print_board():
    for row in range(8):
        for col in range(8):
            color_s = bcolors.INVERT if col+row & 1 == 0 else bcolors.RST_INVERT
            color_e = bcolors.INVERT if col+row & 1 == 1 else bcolors.RST_INVERT
            piece = board[col][row]
            print(color_s + " " + (" " if piece == None else piece.char) + " " + color_e, end="")
        print()
    print(bcolors.RST_INVERT)


def prospect_move(x, y):
    # also return delta in score after this move (ie if move captures)
    return x >= 0 and x < 8 and y >= 0 and y < 8 and board[x][y] == None


def moves_m(piece_x: int, piece_y: int, vects):
    moves = []
    for dx, dy in vects:
        x = piece_x + dx
        y = piece_y + dy
        if prospect_move(x, y):
            moves.append((x, y))
    return moves


def moves_d(piece_x: int, piece_y: int, vects):
    pass
    #for dx, dy in vects:
    #    x = piece.x
    #    y = piece.y
    #    if dx > 0:
    #        while on_board(x, y):
    #            x += dx
    #            y += dy


def compute() -> str:
    """
    Returns best move in algebraic notation
    """
    return "computed move"


def do_move(is_white_move: bool, move: str):
    """
    Manipulates the board as given by "move"
    Args:
        move: the move in algebraic notation
    """
    #print("WHITE" if is_white_move else "BLACK")
    # get what piece is being moved
    # get movement matrix of piece
    # for each piece of type being moved, calculate all moves
    # filter() illegal moves (ie. out of board)
    # filter() moves that does not equal "move"
    # if result is 0, input error. if result > 1, disambiguation needed
    pass
    # manipulate board


def make_mover(fn: Callable[[], None], is_white_move: bool) -> Callable[[], None]:
    def mover():
        move = fn()
        print(move)
        do_move(is_white_move, move)
        print_board()
    return mover


def game_loop(i_am_white: bool) -> Callable[[int], bool]:
    prompt_opponent = "black" if i_am_white else "white"
    prompt_me = "white" if i_am_white else "black"

    opponent = make_mover(lambda: input(prompt_opponent + ": "), not i_am_white)
    fry = make_mover(lambda: print(prompt_me, end=": ") or compute(), i_am_white)

    def once(n: int) -> bool:
        print("move {}:".format(str(n)))
        if i_am_white:
            fry()
        opponent()
        if not i_am_white:
            fry()

    return once


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("color", type=str, choices=["b", "w"],
                        help="selects black or white pieces")
    parser.add_argument("-n", type=int, help="search depth")
    args = parser.parse_args()

    game_cycle = game_loop(args.color == "w")

    for i in itertools.count(start=1):
        if game_cycle(i):
            break

#main()