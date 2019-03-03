/** BEGIN: (MORE) EFFICIENT SEARCH AND PIECE MOVING **/
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
    while (board->pieces[i].pos_1d != needle) {
        if (upper - lower <= 1)
            return NULL; // this happens when needle is not in haystack
        if (needle < board->pieces[i].pos_1d)
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
    int current_sq_1d = map_2d_to_1d(moving_piece->pos);
    int landing_sq_1d = map_2d_to_1d(landing_sq);
    int moving_piece_wr_idx = -1;

    int wr_idx = 0;
    struct piece temp_piece;
    for (int rd_idx = 0; rd_idx < basis_board->len; rd_idx++) {
        // make temporary copy of piece from basis board
        temp_piece = basis_board->pieces[rd_idx];

        /** 
         * case: moving piece 1d index decreases after move.
         * action: store that write index
         */
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
            /** 
             * CULPRIT: when black to move, moving_piece_wr_idx is still -1.
             * FIX: need to update moving piece position right at the beginning,
             * to ensure i get correct 1d index.
             */
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
/** END: (MORE) EFFICIENT SEARCH AND PIECE MOVING **/
