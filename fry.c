
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>

#include "fry.h"
#include "san.h"
#include "al.h"


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
    char full_board[8][8];
    memset(full_board, ' ', 64);

    for (int i = 0; i < board.len; i++)
        full_board[board.pieces[i].pos.x][board.pieces[i].pos.y] 
            = board.pieces[i].def->sym;

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
print_board_list(struct board_list board_list)
{
    for (int i = 0; i < board_list.len; i++) {
        printf("   --------- %d/%d ----------\n", i+1, board_list.len);
        print_board(board_list.boards[i]);
        printf("   ------------------------\n\n");
    }
}

__always_inline void
free_board(struct board board)
{
    free(board.pieces);
}

__always_inline void
free_board_list(struct board_list board_list)
{
    for (int i = 0; i < board_list.len; i++) {
        free_board(board_list.boards[i]);
    }
    free(board_list.boards);
}

__always_inline bool
is_within_bounds(struct vect landing_sq)
{
    return landing_sq.x >= 0 && landing_sq.x < 8 && landing_sq.y >= 0 && landing_sq.y < 8;
}

__always_inline bool
is_enemy(struct piece piece, bool is_white_turn)
{
    return (piece.def->sym < 'a') != is_white_turn; // != used as logical XOR
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

struct piece *
piece_at(struct board * board, struct vect pos)
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
            return NULL; // this happens when needle is not in haystack
        if (needle < temp)
            upper = i;
        else
            lower = i;
        i = lower + ((upper - lower) >> 1);
    }

    return &board->pieces[i];
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
    struct board new_board = {
        .len = 0,
        .sum = 0,
        .white_checks = 0,
        .black_checks = 0,
        .pieces = NULL
    };
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

void
pawn_attack(struct board_list * board_list, struct board * basis_board, 
            struct piece * pawn, struct vect vector, bool is_white_turn)
{
    struct vect landing_sq = pawn->pos;
    landing_sq.x += vector.x;
    landing_sq.y += vector.y;

    struct piece * temp_piece = piece_at(basis_board, landing_sq);
    if (temp_piece && is_enemy(*temp_piece, is_white_turn)) {
        // only available if non empty square is enemy
        board_list->boards[board_list->len++]
            = apply_move_eval(basis_board, pawn, landing_sq, is_white_turn);
    }
}

void
pawn_advance(struct board_list * board_list, struct board * basis_board, 
             struct piece * pawn, struct vect vector, bool is_white_turn)
{
    struct vect landing_sq = pawn->pos;
    landing_sq.y += vector.y;

    struct piece * temp_piece = piece_at(basis_board, landing_sq);
    if (temp_piece == NULL
        && is_within_bounds(landing_sq))
    {
        board_list->boards[board_list->len++]
            = apply_move_eval(basis_board, pawn, landing_sq, is_white_turn);
    }
}

struct board_list
generate_pawn(struct board * basis_board, struct piece * moving_piece, bool is_white_turn)
{
    struct board_list board_list = {
        .len = 0,
        .boards = malloc(moving_piece->def->mvt_len * sizeof(struct board))
    };

    bool in_start_pos = is_white_turn ? moving_piece->pos.y == WP_START_RANK 
                                      : moving_piece->pos.y == BP_START_RANK;

    int delta_y = is_white_turn ? -1 : 1;

    struct vect advance_vector   = { .x = 0, .y =     delta_y };
    struct vect advance_2_vector = { .x = 0, .y = 2 * delta_y };

    struct piece * temp_piece;
    // TODO: check for promotion
    pawn_advance(&board_list, basis_board, moving_piece, advance_vector, is_white_turn);
    if (board_list.len == 0) {
        // if previous move was out of bounds, no other moves will be in bounds
        return board_list;
    }

    // can move 1 more square if in start position
    if (in_start_pos) {
        pawn_advance(&board_list, basis_board, moving_piece, advance_2_vector, is_white_turn);
    }

    // attack moves
    // TODO: generalization refactor + bound checks
    struct vect attack_left_vector  = { .x = -1, .y = delta_y };
    struct vect attack_right_vector = { .x =  1, .y = delta_y };

    pawn_attack(&board_list, basis_board, moving_piece, attack_left_vector, is_white_turn);
    pawn_attack(&board_list, basis_board, moving_piece, attack_right_vector, is_white_turn);

    return board_list;
}

struct board_list
generate_iter(struct board * basis_board, struct piece * moving_piece, bool is_white_turn)
{
    #warning generate_iter() is not implemented
    struct board_list board_list = {
        .len = 0,
        .boards = NULL
    };

    return board_list;
}

struct board_list
generate_abs(struct board * basis_board, struct piece * moving_piece, bool is_white_turn)
{
    #warning generate_abs() is not implemented
    struct board_list board_list = {
        .len = 0,
        .boards = NULL
    };

    return board_list;
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
    else if (moving_piece->def->iter)
        boards = generate_iter(basis_board, moving_piece, is_white_turn);
    else
        boards = generate_abs(basis_board, moving_piece, is_white_turn);

    return boards;
}

/**
 * Finds the pieces that can move to given destination.
 * Return: list of pieces that can move to given destination
 * Args: <in> san_move: the destination
 */
struct piece_list
find_moving_pieces(struct board * basis_board, struct move move, bool is_white_turn)
{
    /**
     * 1. generate all possible boards from this state
     * 2. return the boards that has the moving piece in desired position
     */

    // TODO: handle buffer oveflow
    struct piece_list piece_list = {
        .len = 0,
        .pieces = malloc(8 * sizeof(struct piece))
    };
    struct board_list temp_board_list;
    for (int i = 0; i < basis_board->len; i++) { // for each piece on board

        if (is_enemy(basis_board->pieces[i], is_white_turn))
            continue; // skip; only interested in moves current player can make

        temp_board_list = generate(basis_board, &basis_board->pieces[i], is_white_turn);
        for (int j = 0; j < temp_board_list.len; j++) { // for each board state said piece generates

            struct board temp_board = temp_board_list.boards[j];
            for (int k = 0; k < temp_board.len; k++) { // for each piece in said board state
            // TODO: why does is_enemy retain arguments from first call?
                if (is_enemy(temp_board.pieces[k], is_white_turn))
                    continue; // skip; only interested in moves current player can make
                if (is_same_square(temp_board.pieces[k].pos, move.landing_sq)) {
                    // copy this piece to result list
                    piece_list.pieces[piece_list.len++] = temp_board.pieces[k];
                }
            }
        }
        if (temp_board_list.boards != NULL)
            free_board_list(temp_board_list);
    }

    return piece_list;
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
    //static int boards[MAX_SEARCH_DEPTH][8][8];
    //if (n == 0)
    //    memcpy(&(boards[n]), board, 64 * sizeof(int));
    //else
    //    memcpy(&(boards[n]), &(boards[n-1]), 64 * sizeof(int));

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
    //vect_list_t * al_origins = find_origins(is_white);
    //struct vect * origins = GET_VECTS(al_origins);

    //struct al * moves_list_list = NEW_LIST_LIST();
    //for (int i = 0; i < al_origins->n; i++) {
    //    move_list_t * al_moves = find_moves(origins[i]);
    //    al_add(moves_list_list, &al_moves); // list of list must contain pointers so that they can be individually freed afterwards

    //    struct move * moves = GET_MOVES(al_moves);
    //    for (int j = 0; j < al_moves->n; j++) {
    //        _eval_tree(1, depth, origins[i], moves[j], false);
    //    }
    //}
    ////int val = _eval_tree(0, depth, stand_still, false); // i am maximizing, thus next move down will be minimizing
    //al_free_lstlst(moves_list_list);
    //al_free(al_origins);
    //return NULL;
}

void
game_loop(bool i_am_white)
{
    for(int ply = 0; ; ply++) {
        bool is_black_turn = ply & 1;
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
    print_board(m_board);

    printf("\nfinding moves for e2 pawn:\n");
    struct vect origin = { .x = 4, .y = 6 };
    struct board_list board_list
        = generate_pawn(&m_board, piece_at(&m_board, origin), true);
    print_board_list(board_list);
    free_board_list(board_list);

    printf("algebraic notation:\n");
    struct move san_test_moves[2];
    san_test_moves[0] = san_to_move("Nf3", 0); // white moves
    san_test_moves[1] = san_to_move("c5", 1);  // black moves
    printf("\tNf3 ->\n");
    print_move(san_test_moves[0]);
    printf("\tc5 ->\n");
    print_move(san_test_moves[1]);


    printf("finding the pieces to move:\n");
    struct piece_list moving_pieces = find_moving_pieces(&m_board, san_test_moves[0], true);
    printf("\tNf3 -> from ");
    //print_moves_al(san_test_moves[0].piece, origins);
    //al_free(origins);
    //origins = find_origin(san_test_moves[1]);
    //printf("\tc5 -> from ");
    //print_moves_al(san_test_moves[1].piece, origins);
    //al_free(origins);

    return 0;
}

//int
//interactive()
//{
//    vect_list_t * origins;
//    char readbuf[256];
//    for (int i = 0; ; i++) {
//        //char *fgets(char *s, int size, FILE *stream);
//        //char *strtok(char *str, const char *delim);
//        printf("%d:%c> ", i>>1, i & 1 ? 'b' : 'w');
//        if (fgets(readbuf, 10, stdin) == NULL) {
//            printf("fgets() returned NULL\n");
//            break;
//        }
//        struct san_move san_move = san_to_move(readbuf, i);
//        origins = find_origin(san_move);
//
//        if (origins->n != 1) {
//            printf("%s\n", origins->n == 0 ? "illegal move" : "ambiguous move");
//            i--;
//            //goto flushbuf;
//            continue;
//        }
//
//        struct vect * origin = (struct vect *) al_get(origins, 0);
//        move(board, origin, &(san_move.dest));
//        print_board();
//        al_free(origins);
//    //flushbuf:
//    //    for(int i = 0; i < 10 && getchar() != '\n'; i++);
//    }
//    return 0;
//}

#define TEST 1
#define INTERACTIVE 0

int
main(int argc, char ** argv)
{
    #if TEST == 1
    test();
    #endif

    #if INTERACTIVE == 1
    interactive();
    #endif
    game_loop(true);
    return 0;
}
