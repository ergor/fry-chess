
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


struct m_vect {
    int dx;
    int dy;
};

struct d_vect {
    int dx;
    int dy;
};

struct piece {
    int val;
};

struct pos {
    int x;
    int y;
};

extern struct d_vect q_dvects[8];
extern struct d_vect r_dvects[4];
extern struct d_vect b_vects[4];
extern struct m_vect wp_mvects[3];
extern struct m_vect bp_mvects[3];
extern struct m_vect n_mvects[8];
extern struct m_vect k_mvects[8];

#endif
