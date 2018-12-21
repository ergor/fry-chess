
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
print_moves(int piece, struct vect * moves, int len)
{
    for (int i = 0; i < len; i++)
        printf("%c(%d, %d), ", piece, moves[i].x, moves[i].y);
    printf("\n");
}

/**
 * moves: arraylist of struct pos
 */
void print_moves_al(char piece, struct al * moves)
{
    struct vect * move;
    for (int i = 0; i < moves->n; i++) {
        move = (struct vect *) al_get(moves, i);
        printf("%c(%d, %d), ", piece, move->x, move->y);
    }
    printf("\n");
}


int
prospect_move(struct vect move)
{
    // TODO: return delta in score after this move (ie if move captures)
    return move.x >= 0 && move.x < 8 && move.y >= 0 && move.y < 8 && board[move.x][move.y] == EE;
}

void
absolute_moves(struct vect * origin, struct al * moves, struct piece * piece)
{
    struct vect move;

    for (int i = 0; i < piece->mvt_len; i++) {
        move.x = origin->x + piece->mvt[i].x;
        move.y = origin->y + piece->mvt[i].y;
        if (prospect_move(move)) {
            al_add(moves, &move);
        }
    }
}

void
iterative_moves(struct al * moves, struct piece * piece)
{

}

/**
 * Finds all legal moves
 * Return: moves as arraylist of struct pos
 * Args: <in> m_vects: m_vect movement type vectors
 *       <in>     len: number of elements in m_vects
 */
struct al *
find_moves(struct vect origin)
{
    struct al * moves = al_new(sizeof(struct vect));

    struct piece * piece = &(pieces[board[origin.x][origin.y]]);
    if (piece->iter)
        iterative_moves(moves, piece);
    else
        absolute_moves(&origin, moves, piece);
    
    return moves;
}

void
move(struct vect origin, struct vect dest)
{
    board[dest.x][dest.y] = board[origin.x][origin.y];
    board[origin.x][origin.y] = EE;
}

/**
 * Finds the pieces that can move to given destination.
 * Return: array list of elligible piece positions
 * Args: <in> san_move: the destination
 */
struct al *
find_origin(struct san_move san_move)
{
    int idx = 0;
    struct vect canditates[64];

    for (int y = 0; y < 8; y++) {
        for (int x = 0; x < 8; x++) {
            if (board[x][y] == san_move.piece) {
                canditates[idx].x = x;
                canditates[idx].y = y;
                idx += 1;
            }
        }
    }

    struct al * origins = al_new(sizeof(struct vect));
    struct vect dest = san_move.move;

    struct al * moves; //= al_new(sizeof(struct vect));
    for (int i = 0; i < idx; i++) {
        //m_moves(pos.x, pos.y, vects[san_move.piece]
        moves = find_moves(canditates[i]);
        for (int j = 0; j < moves->n; j++) {
            struct vect * move = (struct vect *) al_get(moves, j);
            if (move->x == dest.x && move->y == dest.y) {
                al_add(origins, &(canditates[i]));
            }
        }
        al_free(moves);
    }

    return origins;
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
    struct vect origin = { .x = 4, .y = 6 };
    //struct al * moves = m_moves(4, 6, wp_mvects, sizeof(wp_mvects)/sizeof(wp_mvects[0]));
    struct al * moves = find_moves(origin);
    print_moves_al(board[4][6], moves);
    al_free(moves);

    printf("algebraic notation:\n");
    struct san_move san_test_moves[2];
    san_test_moves[0] = san_to_move("Nf3", 0);
    san_test_moves[1] = san_to_move("c5", 1);
    printf("\tNf3 -> ");
    print_moves(san_test_moves[0].piece, &(san_test_moves[0].move), 1);
    printf("\tc5 -> ");
    print_moves(san_test_moves[1].piece, &(san_test_moves[1].move), 1);


    printf("finding the pieces to move:\n");
    struct al * origins = find_origin(san_test_moves[0]);
    printf("\tNf3 -> from ");
    print_moves_al(san_test_moves[0].piece, origins);
    al_free(origins);
    origins = find_origin(san_test_moves[1]);
    printf("\tc5 -> from ");
    print_moves_al(san_test_moves[1].piece, origins);
    al_free(origins);

    return 0;
}

int
interactive()
{
    struct al * origins;
    char readbuf[256];
    for (int i = 0; ; i++) {
        //char *fgets(char *s, int size, FILE *stream);
        //char *strtok(char *str, const char *delim);
        printf("%d:%c> ", i>>1, i & 1 ? 'b' : 'w');
        if (fgets(readbuf, 3, stdin) == NULL) {
            printf("fgets() returned NULL\n");
            break;
        }
        struct san_move san_move = san_to_move(readbuf, i);
        origins = find_origin(san_move);

        if (origins->n != 1) {
            printf("%s\n", origins->n == 0 ? "illegal move" : "ambiguous move");
            i--;
            goto flushbuf;
        }

        struct vect * origin = (struct vect *) al_get(origins, 0);
        move(*origin, san_move.move);
        print_board();
        al_free(origins);
    flushbuf:
        for(int i = 0; i < 10 && getchar() != '\n'; i++);
    }
    return 0;
}

#define TEST 1
#define INTERACTIVE 1

int
main(int argc, char ** argv)
{
    init_pieces();
    #if TEST == 1
    test();
    #endif

    #if INTERACTIVE == 1
    interactive();
    #endif
    return 0;
}
