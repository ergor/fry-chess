
#include <stdlib.h>
#include "fry.h"

//------------------------------------------------------------------------------
// QUEEN

#define Q_MVT_LEN   8

struct vect queen_mvt[Q_MVT_LEN] = {
    { .x = -1, .y =  0 }, /* lateral */
    { .x =  1, .y =  0 },
    { .x =  0, .y = -1 },
    { .x =  0, .y =  1 },
    { .x = -1, .y = -1 }, /* diagonal */
    { .x =  1, .y =  1 },
    { .x = -1, .y =  1 },
    { .x =  1, .y = -1 }
};

struct piece_def white_queen = {
    .val = VAL_WQ,
    .iter = true,
    .mvt_len = Q_MVT_LEN,
    .mvt = queen_mvt
};

struct piece_def black_queen = {
    .val = VAL_BQ,
    .iter = true,
    .mvt_len = Q_MVT_LEN,
    .mvt = queen_mvt
};


//------------------------------------------------------------------------------
// ROOK

#define R_MVT_LEN   4

struct vect rook_mvt[R_MVT_LEN] = {
    { .x = -1, .y =  0 },
    { .x =  1, .y =  0 },
    { .x =  0, .y = -1 },
    { .x =  0, .y =  1 }
};

struct piece_def white_rook = {
    .val = VAL_WR,
    .iter = true,
    .mvt_len = R_MVT_LEN,
    .mvt = rook_mvt
};

struct piece_def black_rook = {
    .val = VAL_BR,
    .iter = true,
    .mvt_len = R_MVT_LEN,
    .mvt = rook_mvt
};


//------------------------------------------------------------------------------
// BISHOP

#define B_MVT_LEN   4

struct vect bishop_mvt[B_MVT_LEN] = {
    { .x = -1, .y = -1 },
    { .x =  1, .y =  1 },
    { .x = -1, .y =  1 },
    { .x =  1, .y = -1 }
};

struct piece_def white_bishop = {
    .val = VAL_WB,
    .iter = true,
    .mvt_len = B_MVT_LEN,
    .mvt = bishop_mvt
};

struct piece_def black_bishop = {
    .val = VAL_BB,
    .iter = true,
    .mvt_len = B_MVT_LEN,
    .mvt = bishop_mvt
};


//------------------------------------------------------------------------------
// PAWN

#define P_MVT_LEN   0

struct piece_def white_pawn = {
    .val = VAL_WP,
    .iter = false,
    .mvt_len = P_MVT_LEN,
    .mvt = NULL     /* the pawn is a special case */
};

struct piece_def black_pawn = {
    .val = VAL_BP,
    .iter = false,
    .mvt_len = P_MVT_LEN,
    .mvt = NULL     /* the pawn is a special case */
};


//------------------------------------------------------------------------------
// KNIGHT

#define N_MVT_LEN   8

struct vect knight_mvt[N_MVT_LEN] = {
    { .x = -2, .y = -1 },
    { .x = -2, .y =  1 },
    { .x = -1, .y = -2 },
    { .x = -1, .y =  2 },
    { .x =  1, .y = -2 },
    { .x =  1, .y =  2 },
    { .x =  2, .y = -1 },
    { .x =  2, .y =  1 }
};

struct piece_def white_knight = {
    .val = VAL_WN,
    .iter = false,
    .mvt_len = N_MVT_LEN,
    .mvt = knight_mvt
};

struct piece_def black_knight = {
    .val = VAL_BN,
    .iter = false,
    .mvt_len = N_MVT_LEN,
    .mvt = knight_mvt
};


//------------------------------------------------------------------------------
// KING

#define K_MVT_LEN   8

struct vect king_mvt[K_MVT_LEN] = {
    { .x = -1, .y = -1 },
    { .x =  0, .y = -1 },
    { .x =  1, .y = -1 },
    { .x = -1, .y =  0 },
    { .x =  1, .y =  0 },
    { .x = -1, .y =  1 },
    { .x =  0, .y =  1 },
    { .x =  1, .y =  1 }
};

struct piece_def white_king = {
    .val = VAL_WK,
    .iter = false,
    .mvt_len = K_MVT_LEN,
    .mvt = king_mvt
};

struct piece_def black_king = {
    .val = VAL_BK,
    .iter = false,
    .mvt_len = K_MVT_LEN,
    .mvt = king_mvt
};


//------------------------------------------------------------------------------
// STARTING PIECES

/**
 * Board organization
 * 
 *  0,0 --- 7,0
 *   |   \   |
 *  0,7 --- 7,7
 */
struct piece starting_pieces[STARTING_PIECES_COUNT] = {
    // TOP OF BOARD - BLACK ----------------------------------------------------
    { .x = 0, .y = 0, .def = &black_rook   },
    { .x = 1, .y = 0, .def = &black_knight },
    { .x = 2, .y = 0, .def = &black_bishop },
    { .x = 3, .y = 0, .def = &black_queen  },
    { .x = 4, .y = 0, .def = &black_king   },
    { .x = 5, .y = 0, .def = &black_bishop },
    { .x = 6, .y = 0, .def = &black_knight },
    { .x = 7, .y = 0, .def = &black_rook   },

    { .x = 0, .y = 1, .def = &black_pawn   },
    { .x = 1, .y = 1, .def = &black_pawn   },
    { .x = 2, .y = 1, .def = &black_pawn   },
    { .x = 3, .y = 1, .def = &black_pawn   },
    { .x = 4, .y = 1, .def = &black_pawn   },
    { .x = 5, .y = 1, .def = &black_pawn   },
    { .x = 6, .y = 1, .def = &black_pawn   },
    { .x = 7, .y = 1, .def = &black_pawn   },

    // BOTTOM OF BOARD - WHITE -------------------------------------------------
    { .x = 0, .y = 6, .def = &white_pawn   },
    { .x = 1, .y = 6, .def = &white_pawn   },
    { .x = 2, .y = 6, .def = &white_pawn   },
    { .x = 3, .y = 6, .def = &white_pawn   },
    { .x = 4, .y = 6, .def = &white_pawn   },
    { .x = 5, .y = 6, .def = &white_pawn   },
    { .x = 6, .y = 6, .def = &white_pawn   },
    { .x = 7, .y = 6, .def = &white_pawn   },

    { .x = 0, .y = 7, .def = &white_rook   },
    { .x = 1, .y = 7, .def = &white_knight },
    { .x = 2, .y = 7, .def = &white_bishop },
    { .x = 3, .y = 7, .def = &white_queen  },
    { .x = 4, .y = 7, .def = &white_king   },
    { .x = 5, .y = 7, .def = &white_bishop },
    { .x = 6, .y = 7, .def = &white_knight },
    { .x = 7, .y = 7, .def = &white_rook   },
};
