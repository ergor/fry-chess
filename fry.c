
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

void print_moves_al(char piece, vect_list_t * moves)
{
    struct vect * move;
    for (int i = 0; i < moves->n; i++) {
        move = (struct vect *) al_get(moves, i);
        printf("%c(%d, %d), ", piece, move->x, move->y);
    }
    printf("\n");
}

/**
 * Mutates: move
 */
bool
prospect_move(struct move * move)
{
    struct vect * dest = &(move->dest);
    bool within_bounds = dest->x >= 0 && dest->x < 8 && dest->y >= 0 && dest->y < 8;

    move->delta = 0;

    if (!within_bounds)
        return false;

    int landing_sq = board[dest->x][dest->y];
    
    if (landing_sq != EE) {
        move->delta = -pieces[landing_sq].val;
    }

    return within_bounds;
}

/**
 * Args:
 *  <out> moves: the moves are added to this arraylist
 */
void
absolute_moves(struct vect * origin, move_list_t * moves, struct piece * piece)
{
    struct move move;

    for (int i = 0; i < piece->mvt_len; i++) {
        move.dest.x = origin->x + piece->mvt[i].x;
        move.dest.y = origin->y + piece->mvt[i].y;
        if (prospect_move(&move)) {
            al_add(moves, &move);
        }
    }
}

/**
 * Args:
 *  <out> moves: the moves are added to this arraylist
 */
void
iterative_moves(move_list_t * moves, struct piece * piece)
{

}

/**
 * Args:
 *  <out> moves: the moves are added to this arraylist
 */
void
pawn_moves(struct vect * origin, move_list_t * moves, bool is_white)
{
    bool in_start_pos;
    in_start_pos = is_white ? origin->y == WP_START_RANK : origin->y == BP_START_RANK;

    struct move move;
    int delta_y = (is_white ? -1 : 1);

    // initialize move + do regular move
    move.dest.x = origin->x;
    move.dest.y = origin->y + delta_y;
    if (prospect_move(&move)) {
        // TODO: check for promotion before adding move to arraylist
        al_add(moves, &move);
    }

    // can move 1 more square if in start position
    if (in_start_pos) {
        move.dest.y += delta_y;
        if (prospect_move(&move))
            al_add(moves, &move);
    }

    // attack moves
    move.dest.y = origin->y + delta_y;

    move.dest.x = origin->x - 1; // left
    if (prospect_move(&move) && move.delta != 0)
        al_add(moves, &move);

    move.dest.x = origin->x + 1; // right
    if (prospect_move(&move) && move.delta != 0)
        al_add(moves, &move);
}

/**
 * Finds all legal moves
 * Return: moves as arraylist of struct move
 * Args:
 */
move_list_t *
find_moves(struct vect origin)
{
    move_list_t * moves = NEW_MOVE_LIST();

    struct piece * piece = &(pieces[board[origin.x][origin.y]]);

    if (piece->sym == WP || piece->sym == BP)
        pawn_moves(&origin, moves, piece->sym == WP);
    else if (piece->iter)
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
vect_list_t *
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

    vect_list_t * origins = NEW_VECT_LIST();
    struct vect dest = san_move.dest;

    move_list_t * moves;
    for (int i = 0; i < idx; i++) {
        moves = find_moves(canditates[i]);
        for (int j = 0; j < moves->n; j++) {
            struct move * move = (struct move *) al_get(moves, j);
            if (move->dest.x == dest.x && move->dest.y == dest.y) {
                al_add(origins, &(canditates[i]));
            }
        }
        al_free(moves);
    }

    return origins;
}

/**
 * Args:
 *      n: current search depth
 *  depth: maximum search depth
 */
int
_eval_tree(int n, int depth, struct move move, bool maximize)
{
    // before descending, store board state. ie
    // for each depth, previous board state is stored. (log(n) space for n nodes)
    static int boards[MAX_SEARCH_DEPTH][8][8];
    if (n == 0)
        memcpy(&(boards[n]), board, 64 * sizeof(int));
    else
        memcpy(&(boards[n]), &(boards[n-1]), 64 * sizeof(int));
    
    // boards[n] is the location to store the board mutation currently under evaluation
    // (at this point, boards[n] will be a fresh copy of the parent node, and we apply our move to it)
    /* apply move */

    // then:
    /*
     * 1. generate all next moves at this depth after that move (ie. gen from boards[n])
     * 2. for each move:
     *      1. _eval_tree(boards[n])
     */

    if (n < depth) {
        // recursion
        /* generate moves */
        /* for move in moves: */
        /* return _eval_tree(n+1, depth, move, min/max) */
    }
    else {
        // return
    }
}

int
eval_tree(int depth)
{
    if (depth > MAX_SEARCH_DEPTH) {
        printf("warning: requested depth (%d) exceeds max (%d). scaling back...\n", depth, MAX_SEARCH_DEPTH);
        depth = MAX_SEARCH_DEPTH;
    }

    //struct move {
    //struct vect dest;   /* where this move lands */
    //int delta;          /* board value change if this move is performed */
    //};

    // TODO: base stand_still on the first piece to move (as to not have special case for -1, -1)
    struct move stand_still = { .dest = { .x = -1, .y = -1}, .delta = 0 };
    return _eval_tree(0, depth, stand_still, true);
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
    move_list_t * moves = find_moves(origin);
    print_moves_al(board[4][6], moves);
    al_free(moves);

    printf("algebraic notation:\n");
    struct san_move san_test_moves[2];
    san_test_moves[0] = san_to_move("Nf3", 0); // white moves
    san_test_moves[1] = san_to_move("c5", 1);  // black moves
    printf("\tNf3 -> ");
    print_moves(san_test_moves[0].piece, &(san_test_moves[0].dest), 1);
    printf("\tc5 -> ");
    print_moves(san_test_moves[1].piece, &(san_test_moves[1].dest), 1);


    printf("finding the pieces to move:\n");
    vect_list_t * origins = find_origin(san_test_moves[0]);
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
    vect_list_t * origins;
    char readbuf[256];
    for (int i = 0; ; i++) {
        //char *fgets(char *s, int size, FILE *stream);
        //char *strtok(char *str, const char *delim);
        printf("%d:%c> ", i>>1, i & 1 ? 'b' : 'w');
        if (fgets(readbuf, 10, stdin) == NULL) {
            printf("fgets() returned NULL\n");
            break;
        }
        struct san_move san_move = san_to_move(readbuf, i);
        origins = find_origin(san_move);

        if (origins->n != 1) {
            printf("%s\n", origins->n == 0 ? "illegal move" : "ambiguous move");
            i--;
            //goto flushbuf;
            continue;
        }

        struct vect * origin = (struct vect *) al_get(origins, 0);
        move(*origin, san_move.dest);
        print_board();
        al_free(origins);
    //flushbuf:
    //    for(int i = 0; i < 10 && getchar() != '\n'; i++);
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
