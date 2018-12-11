
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include "fry.h"

int board[8][8] = {
    { BR, BP, EE, EE, EE, EE, WP, WR },
    { BN, BP, EE, EE, EE, EE, WP, WN },
    { BB, BP, EE, EE, EE, EE, WP, WB },
    { BQ, BP, EE, EE, EE, EE, WP, WQ },
    { BK, BP, EE, EE, EE, EE, WP, WK },
    { BB, BP, EE, EE, EE, EE, WP, WB },
    { BN, BP, EE, EE, EE, EE, WP, WN },
    { BR, BP, EE, EE, EE, EE, WP, WR }
};

void
print_board()
{
    for (int y = 0; y < 8; y++) {
        for (int x = 0; x < 8; x++) {
            if ((x+y) & 1)
                printf("%s %c ", RST_INVERT, board[x][y]);
            else
                printf("%s %c ", INVERT, board[x][y]);
        }
        printf("%s\n", RST_INVERT);
    }
}

void
print_moves(struct move * moves, int len)
{
    for (int i = 0; i < len; i++)
        printf("(%d, %d), ", moves[i].x, moves[i].y);
    printf("\n");
}

struct move
san_to_move(char * san)
{
    
}

int
prospect_move(int x, int y)
{
    // TODO: return delta in score after this move (ie if move captures)
    return x >= 0 && x < 8 && y >= 0 && y < 8 && board[x][y] == EE;
}

/**
 * Finds all legal moves
 * Return: number of moves
 * Args: <out>   moves__out: the legal moves
 *       <in>       m_vects: m_vect movement type vector
 *       <in>           len: length of
 */
int
m_moves(struct move ** moves__out, int orig_x, int orig_y, struct m_vect * m_vects, int len)
{
    struct move * moves = malloc(len * sizeof(struct m_vect));

    int i, valid = 0;
    for (i = 0; i < len; i++) {
        int x = orig_x + m_vects[i].dx;
        int y = orig_y + m_vects[i].dy;
        if (prospect_move(x, y)) {
            moves[valid].x = x;
            moves[valid].y = y;
            valid += 1;
        }
    }

    *moves__out = moves;
    return valid;
}

void
game_loop(bool i_am_white)
{

}

int
test()
{
    print_board();
    struct move * moves;
    int l = m_moves(&moves, 4, 7, k_mvects, sizeof(k_mvects)/sizeof(k_mvects[0]));
    print_moves(moves, l);
    free(moves);
    return 0;
}

#define TEST 1

int
main(int argc, char ** argv)
{
    #if TEST == 1
    test();
    return 0;
    #endif

    return 0;
}
