
#include <stdlib.h>
#include "fry.h"

struct piece pieces[128]; /* all pieces indexed by their ASCII representation */

//------------------------------------------------------------------------------
// QUEEN

struct vect queen_mvt[] = {
    { .x = -1, .y =  0 }, /* lateral */
    { .x =  1, .y =  0 },
    { .x =  0, .y = -1 },
    { .x =  0, .y =  1 },
    { .x = -1, .y = -1 }, /* diagonal */
    { .x =  1, .y =  1 },
    { .x = -1, .y =  1 },
    { .x =  1, .y = -1 }
};

struct piece queen = {
    .val = 900,
    .iter = true,
    .mvt_len = 8,
    .mvt = queen_mvt,
    .att = NULL
};


//------------------------------------------------------------------------------
// ROOK

struct vect rook_mvt[] = {
    { .x = -1, .y =  0 },
    { .x =  1, .y =  0 },
    { .x =  0, .y = -1 },
    { .x =  0, .y =  1 }
};

struct piece rook = {
    .val = 500,
    .iter = true,
    .mvt_len = 4,
    .mvt = rook_mvt,
    .att = NULL
};


//------------------------------------------------------------------------------
// BISHOP

struct vect bishop_mvt[] = {
    { .x = -1, .y = -1 },
    { .x =  1, .y =  1 },
    { .x = -1, .y =  1 },
    { .x =  1, .y = -1 }
};

struct piece bishop = {
    .val = 300,
    .iter = true,
    .mvt_len = 4,
    .mvt = bishop_mvt,
    .att = NULL
};


//------------------------------------------------------------------------------
// WHITE PAWN

struct vect white_pawn_mvt[] = {
    { .x =  0, .y = -1 }
};

struct vect white_pawn_att[] = {
    { .x = -1, .y = -1 },
    { .x =  1, .y = -1 }
};

struct piece white_pawn = {
    .val = 100,
    .iter = false,
    .mvt_len = 1,
    .att_len = 2,
    .mvt = white_pawn_mvt,
    .att = white_pawn_att
};


//------------------------------------------------------------------------------
// BLACK PAWN

struct vect black_pawn_mvt[] = {
    { .x =  0, .y =  1 }
};

struct vect black_pawn_att[] = {
    { .x = -1, .y =  1 },
    { .x =  1, .y =  1 }
};

struct piece black_pawn = {
    .val = 100,
    .iter = false,
    .mvt_len = 1,
    .att_len = 2,
    .mvt = black_pawn_mvt,
    .att = black_pawn_att
};


//------------------------------------------------------------------------------
// KNIGHT

struct vect knight_mvt[] = {
    { .x = -2, .y = -1 },
    { .x = -2, .y =  1 },
    { .x = -1, .y = -2 },
    { .x = -1, .y =  2 },
    { .x =  1, .y = -2 },
    { .x =  1, .y =  2 },
    { .x =  2, .y = -1 },
    { .x =  2, .y =  1 }
};

struct piece knight = {
    .val = 300,
    .iter = false,
    .mvt_len = 8,
    .mvt = knight_mvt,
    .att = NULL
};


//------------------------------------------------------------------------------
// KING

struct vect king_mvt[] = {
    { .x = -1, .y = -1 },
    { .x =  0, .y = -1 },
    { .x =  1, .y = -1 },
    { .x = -1, .y =  0 },
    { .x =  1, .y =  0 },
    { .x = -1, .y =  1 },
    { .x =  0, .y =  1 },
    { .x =  1, .y =  1 }
};

struct piece king = {
    .val = 100000,
    .iter = false,
    .mvt_len = 8,
    .mvt = king_mvt,
    .att = NULL
};


//------------------------------------------------------------------------------

void init_pieces()
{
    pieces[WK] = king;          /* white king */
    pieces[WK].sym = WK;
    pieces[BK] = king;          /* black king */
    pieces[BK].sym = BK;

    pieces[WQ] = queen;         /* white queen */
    pieces[WQ].sym = WQ;
    pieces[BQ] = queen;         /* black queen */
    pieces[BQ].sym = BQ;

    pieces[WR] = rook;          /* white rook */
    pieces[WR].sym = WR;
    pieces[BR] = rook;          /* black rook */
    pieces[BR].sym = BR;

    pieces[WN] = knight;        /* white knight */
    pieces[WN].sym = WN;
    pieces[BN] = knight;        /* black knight */
    pieces[BN].sym = BN;

    pieces[WB] = bishop;        /* white bishop */
    pieces[WB].sym = WB;
    pieces[BB] = bishop;        /* black bishop */
    pieces[BB].sym = BB;

    pieces[WP] = white_pawn;    /* white pawn */
    pieces[WP].sym = WP;
    pieces[BP] = black_pawn;    /* black pawn */
    pieces[BP].sym = BP;
}
