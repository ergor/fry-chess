
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>

#include "fry.h"
#include "san.h"
#include "al.h"

/* organized as [x][y] */
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
print_board_files()
{
    printf("   ");
    for (int x = 0; x < 8; x++)
        printf(" %c ", 'a' + x);
    printf("\n");
}

void
print_board()
{
    print_board_files();

    for (int y = 0; y < 8; y++) {
        for (int x = -1; x <= 8; x++) {

            if (x == -1 || x == 8) { /* print ranks */
                printf("%s %c ", RST_INVERT, (7-y)+'0'+1);
                continue;
            }

            if ((x+y) & 1)
                printf("%s %c ", RST_INVERT, board[x][y]);
            else
                printf("%s %c ", INVERT, board[x][y]);

        }
        printf("%s\n", RST_INVERT);
    }

    print_board_files();
}

void
print_moves(int piece, struct pos * moves, int len)
{
    for (int i = 0; i < len; i++)
        printf("%c(%d, %d), ", piece, moves[i].x, moves[i].y);
    printf("\n");
}

/**
 * moves: arraylist of struct pos
 */
void print_moves_al(int piece, struct al * moves)
{
    struct pos * move;
    for (int i = 0; i < moves->n; i++) {
        move = (struct pos *) al_get(moves, i);
        printf("%c(%d, %d), ", piece, move->x, move->y);
    }
    printf("\n");
}


int
prospect_move(struct pos move)
{
    // TODO: return delta in score after this move (ie if move captures)
    return move.x >= 0 && move.x < 8 && move.y >= 0 && move.y < 8 && board[move.x][move.y] == EE;
}

/**
 * Finds all legal moves
 * Return: moves as arraylist of struct pos
 * Args: <in> m_vects: m_vect movement type vectors
 *       <in>     len: number of elements in m_vects
 */
struct al *
m_moves(int orig_x, int orig_y, struct m_vect * m_vects, int len)
{
    //struct pos * moves = malloc(len * sizeof(struct m_vect));
    struct pos move;
    struct al * list = al_new(sizeof(struct pos));

    for (int i = 0; i < len; i++) {
        int x = orig_x + m_vects[i].dx;
        int y = orig_y + m_vects[i].dy;
        move.x = x;
        move.y = y;
        if (prospect_move(move)) {
            //moves[valid].piece = board[orig_x][orig_y];
            //moves[valid].x = x;
            //moves[valid].y = y;
            al_add(list, &move);
        }
    }

    return list;
}

/**
 * Finds correct piece to move given a destination.
 * Return: whether move is unambiguous
 * Args: <out> origin__out: the piece that is to be moved
 *       <in>     san_move: the destination
 */
bool
find_origin(struct pos * origin__out, struct san_move san_move)
{
    int idx = 0;
    struct pos pos[64];

    for (int y = 0; y < 8; y++) {
        for (int x = 0; x < 8; x++) {
            if (board[x][y] == san_move.piece) {
                pos[idx].x = x;
                pos[idx].y = y;
                idx += 1;
            }
        }
    }

    struct al * moves = al_new(sizeof(struct pos));
    for (int i = 0; i < idx; i++) {
        //m_moves(pos.x, pos.y, vects[san_move.piece]
    }
}

void
game_loop(bool i_am_white)
{

}

int
test()
{
    printf("board:\n");
    print_board();

    printf("finding moves for e2 pawn:\n\t");
    struct al * moves = m_moves(4, 6, wp_mvects, sizeof(wp_mvects)/sizeof(wp_mvects[0]));
    print_moves_al(board[4][6], moves);
    al_free(moves);

    printf("algebraic notation:\n");
    struct san_move san_test_moves[2];
    san_test_moves[0] = san_to_move("Nf3");
    san_test_moves[1] = san_to_move("c5");
    printf("\tNf3 -> ");
    print_moves(san_test_moves[0].piece, &(san_test_moves[0].move), 1);
    printf("\tc5 -> ");
    print_moves(san_test_moves[1].piece, &(san_test_moves[1].move), 1);

    struct pos origin;
    find_origin(&origin, san_test_moves[0]);
    print_moves(board[origin.x][origin.y], &origin, 1);
    find_origin(&origin, san_test_moves[1]);
    print_moves(board[origin.x][origin.y], &origin, 1);

    return 0;
}

#define TEST 1

int
main(int argc, char ** argv)
{
    init_vects();
    #if TEST == 1
    test();
    return 0;
    #endif

    return 0;
}
