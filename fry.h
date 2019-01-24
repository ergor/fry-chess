
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

#define VAL_BP  -100    /* black pawn   */
#define VAL_BK  -99999  /* black king   */
#define VAL_BQ  -900    /* black queen  */
#define VAL_BR  -500    /* black rook   */
#define VAL_BN  -300    /* black knight */
#define VAL_BB  -300    /* black bishop */

#define WP_START_RANK   6
#define BP_START_RANK   1
#define STARTING_PIECES_COUNT   32

#define MAX_SEARCH_DEPTH    64

struct vect {
    int x;
    int y;
};

struct move {
    struct vect dest;   /* where this move lands */
    int delta;          /* board value change if this move is performed */
};

struct piece_def {
    int  val;                /* centipawn value of piece */
    char sym;               /* ASCII representation of the piece */
    bool iter;              /* whether the vector should be iterated */
    int  mvt_len;            /* movement vecetor length */
    struct vect * mvt;      /* movement vector */
};

struct piece {
    int x;
    int y;
    struct piece_def * def;
};

struct board {
    int len;
    int white_checks;   /* number of checks against white king */
    int black_checks;   /* number of checks against black king */
    struct piece * pieces;
};

extern struct piece starting_pieces[32];

#endif
