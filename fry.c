
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>

#include "fry.h"
#include "san.h"
#include "al.h"


/**
 * Board organization
 * 
 *  0,0 --- 7,0 
 *   |   \   |
 *  0,7 --- 7,7 
 */
int full_board[8][8] = {
    { BR, BP, EE, EE, EE, EE, WP, WR },
    { BN, BP, EE, EE, EE, EE, WP, WN },
    { BB, BP, EE, EE, EE, EE, WP, WB },
    { BQ, BP, EE, EE, EE, EE, WP, WQ },
    { BK, BP, EE, EE, EE, EE, WP, WK },
    { BB, BP, EE, EE, EE, EE, WP, WB },
    { BN, BP, EE, EE, EE, EE, WP, WN },
    { BR, BP, EE, EE, EE, EE, WP, WR }
};

struct board m_board = {
    .len = STARTING_PIECES_COUNT,
    .pieces = starting_pieces
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
print_board(struct board board)
{
    int full_board[8][8];
    memset(full_board, ' ', 64);

    for (int i = 0; i < board.len; i++)
        full_board[board.pieces[i].x][board.pieces[i].y] = board.pieces[i].def->sym;

    print_board_files();

    for (int y = 0; y < 8; y++) {
        for (int x = -1; x <= 8; x++) {

            if (x == -1 || x == 8) { /* print ranks */
                printf("%s %c ", RST_INVERT, (7-y)+'0'+1);
                continue;
            }

            if ((x+y) & 1)
                printf("%s %c ", RST_INVERT, full_board[x][y]);
            else
                printf("%s %c ", INVERT, full_board[x][y]);

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


// /**
//  * Mutates: move
//  */
// bool
// prospect_move(struct move * move)
// {
//     struct vect * dest = &(move->dest);
//     bool within_bounds = dest->x >= 0 && dest->x < 8 && dest->y >= 0 && dest->y < 8;
// 
//     move->delta = 0;
// 
//     if (!within_bounds)
//         return false;
// 
//     int landing_sq = board[dest->x][dest->y];
// 
//     if (landing_sq != EE) {
//         move->delta = -pieces[landing_sq].val;
//     }
// 
//     return within_bounds;
// }

// /**
//  * Args:
//  *  <out> moves: the moves are added to this arraylist
//  */
// void
// absolute_moves(struct vect * origin, move_list_t * moves, struct piece * piece)
// {
//     struct move move;
// 
//     for (int i = 0; i < piece->mvt_len; i++) {
//         move.dest.x = origin->x + piece->mvt[i].x;
//         move.dest.y = origin->y + piece->mvt[i].y;
//         if (prospect_move(&move)) {
//             al_add(moves, &move);
//         }
//     }
// }

// /**
//  * Args:
//  *  <out> moves: the moves are added to this arraylist
//  */
// void
// iterative_moves(move_list_t * moves, struct piece * piece)
// {
// 
// }

// /**
//  * Finds all legal moves
//  * Return: moves as arraylist of struct move
//  * Args:
//  */
// move_list_t *
// find_moves(struct vect origin)
// {
//     move_list_t * moves = NEW_MOVE_LIST();

//     struct piece * piece = &(pieces[board[origin.x][origin.y]]);

//     if (piece->sym == WP || piece->sym == BP)
//         pawn_moves(&origin, moves, piece->sym == WP);
//     else if (piece->iter)
//         iterative_moves(moves, piece);
//     else
//         absolute_moves(&origin, moves, piece);

//     return moves;
// }

/**
 * evaluation function
 * 
 * input: the board, ie. list of pieces
 *      where: piece = {x,y,value}
 * 
 * output: value of given board
 */
int
evaluate(struct board board)
{
    int sum = 0;
    for (int i = 0; i < board.len; i++) {
        sum += board.pieces[i].def->val;
    }
    return sum;
}

__always_inline bool
is_within_bounds(struct vect landing_sq)
{
    return landing_sq.x >= 0 && landing_sq.x < 8 && landing_sq.y >= 0 && landing_sq.y < 8;
}

__always_inline bool
is_enemy(struct piece piece, bool is_white_turn)
{

}

__always_inline bool
is_same_square(struct vect pos1, struct vect pos2)
{
    return pos1.x == pos2.x && pos1.y == pos2.y;
}

__always_inline int
map_2d_to_1d(struct vect pos)
{
    return pos.y * 8 + pos.x;
}

int
piece_index_at(struct board * board, struct vect pos)
{
    /** 
     * search for piece in the board:
     * - the pieces in the list maintain sorted order (0,0=>0; 7;7=>63)
     * - tree search:
     *  - split search area and get value of element in the intersection
     *  - if needle == element: break
     *  - if needle < element: tree search lower half
     *  - if needle > element: tree search upper half
     */
    int upper = board->len;
    int lower = 0;
    int i = upper >> 1;

    int needle = map_2d_to_1d(pos);
    int temp;
    while ((temp = map_2d_to_1d(board->pieces[i].pos)) != needle) {
        if (upper - lower <= 1)
            return -1; // this happens when needle is not in haystack
        if (needle < temp)
            upper = i;
        else
            lower = i;
        i = lower + ((upper - lower) >> 1);
    }

    return i;
}

/**
 * generates and evaluates a new board state
 */
struct board
apply_move_eval(struct board * basis_board, struct piece * moving_piece, 
                struct vect landing_sq, bool is_white_turn)
{
    /**
     * 1. allocate memory for pieces of board under construction
     * 2. copy pieces from basis board:
     *  - for each piece in basis board:
     *      - if the moving piece:
     *          - update position and copy
     *      - if piece is enemy and position == landing square:
     *          - skip copy
     *      - else: copy
     */
    struct board new_board;
    struct piece * new_piece_arr = malloc(basis_board->len * sizeof(struct piece));

    // for ensuring sorted order:
    int landing_sq_1d = map_2d_to_1d(landing_sq);
    int moving_piece_wr_idx = -1;

    int wr_idx = 0;
    struct piece temp_piece;
    for (int rd_idx = 0; rd_idx < basis_board->len; rd_idx++) {
        // make temporary copy of piece from basis board
        temp_piece = basis_board->pieces[rd_idx];

        if(moving_piece_wr_idx == -1
           && map_2d_to_1d(temp_piece.pos) >= landing_sq_1d)
        {
            moving_piece_wr_idx = wr_idx;
            wr_idx += 1;
        }

        if (is_same_square(temp_piece.pos, landing_sq)) {
            /**
             * this case will never occur if landing_sq is an empty square.
             * thus if this case fires, the square is NOT empty, AND:
             * assuming landing_sq is always legal, then:
             * temp_piece will always be an enemy, and we have a capture.
             * don't copy the enemy piece to the new board state.
             */
            continue;
        }

        if (is_same_square(temp_piece.pos, moving_piece->pos)) {
            // if the moving piece, apply the move
            temp_piece.pos = landing_sq;
            new_piece_arr[moving_piece_wr_idx] = temp_piece;
            goto finally;
        }

        // copy temp piece into new array and update board evaluation
        new_piece_arr[wr_idx++] = temp_piece;
    finally:
        new_board.sum += temp_piece.def->val;

        // TODO: count checks
    }

    new_board.len = wr_idx;
    new_board.pieces = new_piece_arr;

    return new_board;
}

struct board_list
generate_pawn(struct board * basis_board, struct piece * moving_piece, bool is_white_turn)
{
    struct board_list boards = {
        .len = 0,
        .boards = malloc(moving_piece->def->mvt_len * sizeof(struct board))
    };

    bool in_start_pos = is_white_turn ? moving_piece->pos.y == WP_START_RANK 
                                      : moving_piece->pos.y == BP_START_RANK;

    struct vect landing_sq;
    int delta_y = is_white_turn ? -1 : 1;

    // initialize move + do regular move
    landing_sq = moving_piece->pos;
    landing_sq.y += delta_y;
    int piece_idx;
    if (is_within_bounds(landing_sq)) {
        // TODO: check for promotion before adding move to arraylist
        piece_idx = piece_index_at(basis_board, landing_sq);
        if (piece_idx < 0 || is_enemy(basis_board->pieces[piece_idx], is_white_turn)) {
            // landing_sq is an empty square or an enemy
            boards.boards[boards.len] = apply_move_eval(basis_board, moving_piece, landing_sq, is_white_turn);
            boards.len += 1;
        }
    }

    // can move 1 more square if in start position
    if (in_start_pos) {
        landing_sq.y += delta_y;
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
 * board generator
 * 
 * if return is empty list, then no legal moves can be made:
 *      if number of checks > 0: check mate
 *      else: stalemate
 * 
 * input: the piece to move in given board state
 *      where: board = {pieces}
 *             piece = {x,y,value}
 * 
 * output: list of possible boards
 */
struct board_list
generate(struct board * basis_board, struct piece * moving_piece, bool is_white_turn)
{
    /**
     * 1. generate all possible board states
     * 2. discard illegal states:
     *      - piece lands on/passes through own
     *      - piece lands out of bounds
     *      - board states where im in check
     * 3. count checks against enemy
     */

    struct board_list boards;

    if (moving_piece->def->sym == WP || moving_piece->def->sym == BP)
        boards = generate_pawn(basis_board, moving_piece, is_white_turn);
    //else if (piece->iter)
    //    iterative_moves(moves, piece);
    //else
    //    absolute_moves(&origin, moves, piece);

    return boards;
}

//void
//move(int board[8][8], struct vect * origin, struct vect * dest)
//{
//    board[dest->x][dest->y] = board[origin->x][origin->y];
//    board[origin->x][origin->y] = EE;
//}

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

vect_list_t *
find_origins(bool white)
{
    vect_list_t * origins = NEW_VECT_LIST();
    char * syms = white ? "PQKRNB" : "pqkrnb";

    struct vect tmp;
    for (int y = 0; y < 8; y++) {
        for (int x = 0; x < 8; x++) {
            if (strchr(syms, board[x][y]) != NULL) {
                tmp.x = x;
                tmp.y = y;
                al_add(origins, &tmp);
            }
        }
    }

    return origins;
}

/**
 * Args:
 *      n: current search depth
 *  depth: maximum search depth
 */
int
_eval_tree(int n, int depth, struct vect origin, struct move move, bool maximize)
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

struct move *
eval_tree(int depth, bool is_white)
{
    if (depth > MAX_SEARCH_DEPTH) {
        printf("warning: requested depth (%d) exceeds max (%d). scaling back...\n", depth, MAX_SEARCH_DEPTH);
        depth = MAX_SEARCH_DEPTH;
    }

    //struct move {
    //struct vect dest;   /* where this move lands */
    //int delta;          /* board value change if this move is performed */
    //};

    /*
     * 1. generate all possible moves from this position (ie the begining of my turn)
     * 2. for each possible move:
     *      1. _eval_tree(move)
     *      2. store result
     * 3. return best move
     * */

    // TODO: base stand_still on the first piece to move (as to not have special case for -1, -1)
    vect_list_t * al_origins = find_origins(is_white);
    struct vect * origins = GET_VECTS(al_origins);

    struct al * moves_list_list = NEW_LIST_LIST();
    for (int i = 0; i < al_origins->n; i++) {
        move_list_t * al_moves = find_moves(origins[i]);
        al_add(moves_list_list, &al_moves); // list of list must contain pointers so that they can be individually freed afterwards

        struct move * moves = GET_MOVES(al_moves);
        for (int j = 0; j < al_moves->n; j++) {
            _eval_tree(1, depth, origins[i], moves[j], false);
        }
    }
    //int val = _eval_tree(0, depth, stand_still, false); // i am maximizing, thus next move down will be minimizing
    al_free_lstlst(moves_list_list);
    al_free(al_origins);
    return NULL;
}

void
game_loop(bool i_am_white)
{
    for(int i = 0; ; i++) {
        bool is_black_turn = i & 1;
        if (i_am_white) {
            if (is_black_turn) {

            }
            else {
                eval_tree(10, true);
            }
        }
    }
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
        move(board, origin, &(san_move.dest));
        print_board();
        al_free(origins);
    //flushbuf:
    //    for(int i = 0; i < 10 && getchar() != '\n'; i++);
    }
    return 0;
}

#define TEST 1
#define INTERACTIVE 0

int
main(int argc, char ** argv)
{
    init_piece_defs();
    #if TEST == 1
    test();
    #endif

    #if INTERACTIVE == 1
    interactive();
    #endif
    game_loop(true);
    return 0;
}
