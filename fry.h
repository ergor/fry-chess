
#ifndef FRY_H
#define FRY_H

#include <stdbool.h>

#define INVERT      "\033[7m"
#define RST_INVERT  "\033[27m"

#define EE      ' '     /* empty square */

#define WP      'P'     /* white pawn   */
#define WK      'K'     /* white king   */
#define WQ      'Q'     /* white queen  */
#define WR      'R'     /* white rook   */
#define WN      'N'     /* white knight */
#define WB      'B'     /* white bishop */

#define BP      'p'     /* black pawn   */
#define BK      'k'     /* black king   */
#define BQ      'q'     /* black queen  */
#define BR      'r'     /* black rook   */
#define BN      'n'     /* black knight */
#define BB      'b'     /* black bishop */

#define VAL_WP  100     /* white pawn   */
#define VAL_WK  99999   /* white king   */
#define VAL_WQ  900     /* white queen  */
#define VAL_WR  500     /* white rook   */
#define VAL_WN  300     /* white knight */
#define VAL_WB  300     /* white bishop */

#define VAL_BP  -VAL_WP     /* black pawn   */
#define VAL_BK  -VAL_WK     /* black king   */
#define VAL_BQ  -VAL_WQ     /* black queen  */
#define VAL_BR  -VAL_WR     /* black rook   */
#define VAL_BN  -VAL_WN     /* black knight */
#define VAL_BB  -VAL_WB     /* black bishop */

#define WP_START_RANK   6
#define BP_START_RANK   1
#define STARTING_PIECES_COUNT   32

#define MAX_SEARCH_DEPTH    64

struct vect {
    int x;
    int y;
};

struct piece_def {
    int  val;                /* centipawn value of piece */
    char sym;               /* ASCII representation of the piece */
    bool iter;              /* whether the vector should be iterated */
    int  mvt_len;            /* movement vecetor length */
    struct vect * mvt;      /* movement vector */
};

struct piece {
    struct vect pos;
    struct piece_def * def;
};

/* a light weight cousin to struct board */
struct piece_list {
    int len;
    struct piece * pieces;
};

struct board {
    int len;                /* number of pieces on the board */
    int sum;                /* board evaluation */
    int white_checks;       /* number of checks against white king */
    int black_checks;       /* number of checks against black king */
    struct piece * pieces;  /* the pieces on the board (consecutive) */
};

struct board_list {
    int len;                /* number of boards */
    struct board * boards;  /* the boards (consecutive) */
};

extern struct piece starting_pieces[32];

#endif
