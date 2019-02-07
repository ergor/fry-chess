
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
    .sym = WQ,
    .val = VAL_WQ,
    .iter = true,
    .mvt_len = Q_MVT_LEN,
    .mvt = queen_mvt
};

struct piece_def black_queen = {
    .sym = BQ,
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
    .sym = WR,
    .val = VAL_WR,
    .iter = true,
    .mvt_len = R_MVT_LEN,
    .mvt = rook_mvt
};

struct piece_def black_rook = {
    .sym = BR,
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
    .sym = WB,
    .val = VAL_WB,
    .iter = true,
    .mvt_len = B_MVT_LEN,
    .mvt = bishop_mvt
};

struct piece_def black_bishop = {
    .sym = BB,
    .val = VAL_BB,
    .iter = true,
    .mvt_len = B_MVT_LEN,
    .mvt = bishop_mvt
};


//------------------------------------------------------------------------------
// PAWN

#define P_MVT_LEN   4

struct piece_def white_pawn = {
    .sym = WP,
    .val = VAL_WP,
    .iter = false,
    .mvt_len = P_MVT_LEN,
    .mvt = NULL     /* the pawn is a special case */
};

struct piece_def black_pawn = {
    .sym = BP,
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
    .sym = WN,
    .val = VAL_WN,
    .iter = false,
    .mvt_len = N_MVT_LEN,
    .mvt = knight_mvt
};

struct piece_def black_knight = {
    .sym = BN,
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
    .sym = WK,
    .val = VAL_WK,
    .iter = false,
    .mvt_len = K_MVT_LEN,
    .mvt = king_mvt
};

struct piece_def black_king = {
    .sym = BK,
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
    { .index = 0, .pos_1d = 0,   { .x = 0, .y = 0 }, .def = &black_rook   },
    { .index = 1, .pos_1d = 1,   { .x = 1, .y = 0 }, .def = &black_knight },
    { .index = 2, .pos_1d = 2,   { .x = 2, .y = 0 }, .def = &black_bishop },
    { .index = 3, .pos_1d = 3,   { .x = 3, .y = 0 }, .def = &black_queen  },
    { .index = 4, .pos_1d = 4,   { .x = 4, .y = 0 }, .def = &black_king   },
    { .index = 5, .pos_1d = 5,   { .x = 5, .y = 0 }, .def = &black_bishop },
    { .index = 6, .pos_1d = 6,   { .x = 6, .y = 0 }, .def = &black_knight },
    { .index = 7, .pos_1d = 7,   { .x = 7, .y = 0 }, .def = &black_rook   },

    { .index = 8, .pos_1d = 8,   { .x = 0, .y = 1 }, .def = &black_pawn   },
    { .index = 9, .pos_1d = 9,   { .x = 1, .y = 1 }, .def = &black_pawn   },
    { .index = 10, .pos_1d = 10, { .x = 2, .y = 1 }, .def = &black_pawn   },
    { .index = 11, .pos_1d = 11, { .x = 3, .y = 1 }, .def = &black_pawn   },
    { .index = 12, .pos_1d = 12, { .x = 4, .y = 1 }, .def = &black_pawn   },
    { .index = 13, .pos_1d = 13, { .x = 5, .y = 1 }, .def = &black_pawn   },
    { .index = 14, .pos_1d = 14, { .x = 6, .y = 1 }, .def = &black_pawn   },
    { .index = 15, .pos_1d = 15, { .x = 7, .y = 1 }, .def = &black_pawn   },

    // BOTTOM OF BOARD - WHITE -------------------------------------------------
    { .index = 16, .pos_1d = 48, { .x = 0, .y = 6 }, .def = &white_pawn   },
    { .index = 17, .pos_1d = 49, { .x = 1, .y = 6 }, .def = &white_pawn   },
    { .index = 18, .pos_1d = 50, { .x = 2, .y = 6 }, .def = &white_pawn   },
    { .index = 19, .pos_1d = 51, { .x = 3, .y = 6 }, .def = &white_pawn   },
    { .index = 20, .pos_1d = 52, { .x = 4, .y = 6 }, .def = &white_pawn   },
    { .index = 21, .pos_1d = 53, { .x = 5, .y = 6 }, .def = &white_pawn   },
    { .index = 22, .pos_1d = 54, { .x = 6, .y = 6 }, .def = &white_pawn   },
    { .index = 23, .pos_1d = 55, { .x = 7, .y = 6 }, .def = &white_pawn   },

    { .index = 24, .pos_1d = 56, { .x = 0, .y = 7 }, .def = &white_rook   },
    { .index = 25, .pos_1d = 57, { .x = 1, .y = 7 }, .def = &white_knight },
    { .index = 26, .pos_1d = 58, { .x = 2, .y = 7 }, .def = &white_bishop },
    { .index = 27, .pos_1d = 59, { .x = 3, .y = 7 }, .def = &white_queen  },
    { .index = 28, .pos_1d = 60, { .x = 4, .y = 7 }, .def = &white_king   },
    { .index = 29, .pos_1d = 61, { .x = 5, .y = 7 }, .def = &white_bishop },
    { .index = 30, .pos_1d = 62, { .x = 6, .y = 7 }, .def = &white_knight },
    { .index = 31, .pos_1d = 63, { .x = 7, .y = 7 }, .def = &white_rook   },
};
