
#ifndef FRY_H
#define FRY_H

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


struct vect {
    int x;
    int y;
};

typedef struct vect m_vect_t;
typedef struct vect d_vect_t;

struct m_vect {
    int dx;
    int dy;
};

struct d_vect {
    int dx;
    int dy;
};

struct move {
    int piece;
    int x;
    int y;
};

struct piece {
    int val;
};

struct d_vect
q_dvects[] = {
    { .dx = -1, .dy =  0 }, /* lateral */
    { .dx =  1, .dy =  0 },
    { .dx =  0, .dy = -1 },
    { .dx =  0, .dy =  1 },
    { .dx = -1, .dy = -1 }, /* diagonal */
    { .dx =  1, .dy =  1 },
    { .dx = -1, .dy =  1 },
    { .dx =  1, .dy = -1 }
};

struct d_vect
r_dvects[4] = {
    { .dx = -1, .dy =  0 },
    { .dx =  1, .dy =  0 },
    { .dx =  0, .dy = -1 },
    { .dx =  0, .dy =  1 }
};

struct d_vect
b_vects[] = {
    { .dx = -1, .dy = -1 },
    { .dx =  1, .dy =  1 },
    { .dx = -1, .dy =  1 },
    { .dx =  1, .dy = -1 }
};

struct m_vect
wp_mvects[] = {
    { .dx =  0, .dy =  0 },
    { .dx = -1, .dy = -1 },
    { .dx = -0, .dy = -1 },
    { .dx =  1, .dy = -1 }
};

struct m_vect
bp_mvects[] = {
    { .dx =  0, .dy =  0 },
    { .dx = -1, .dy =  1 },
    { .dx = -0, .dy =  1 },
    { .dx =  1, .dy =  1 }
};

struct m_vect
n_mvects[] = {
    { .dx =  0, .dy =  0 },
    { .dx = -2, .dy = -1 },
    { .dx = -2, .dy =  1 },
    { .dx = -1, .dy = -2 },
    { .dx = -1, .dy =  2 },
    { .dx =  1, .dy = -2 },
    { .dx =  1, .dy =  2 },
    { .dx =  2, .dy = -1 },
    { .dx =  2, .dy =  1 }
};

struct m_vect
k_mvects[] = {
    { .dx = -1, .dy = -1 },
    { .dx =  0, .dy = -1 },
    { .dx =  1, .dy = -1 },
    { .dx = -1, .dy =  0 },
    { .dx =  0, .dy =  0 },
    { .dx =  1, .dy =  0 },
    { .dx = -1, .dy =  1 },
    { .dx =  0, .dy =  1 },
    { .dx =  1, .dy =  1 }
};

#endif
