
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <sys/types.h>
#include <regex.h>

#include "fry.h"

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
print_moves(struct move * moves, int len)
{
    for (int i = 0; i < len; i++)
        printf("%c(%d, %d), ", moves[i].piece, moves[i].x, moves[i].y);
    printf("\n");
}

struct move
san_to_move(char * san)
{
    //int len = strlen(san);
    struct move move = { .x = -1, .y = -1 };
    char msgbuf[1024];
    regex_t regex;
    size_t nmatch = 3; /* 0: the whole match, 1: capture group 1, 2: capture group 2 */
    regmatch_t match[nmatch];

    int reti = regcomp(&regex, "([RNBQK])*x*([a-h][1-8])", REG_EXTENDED);
    if (reti) {
        fprintf(stderr, "failed to compile regex\n");
        exit(reti);
    }

    reti = regexec(&regex, san, nmatch, match, 0);
    if (reti == 0) {
        char piece = 'P';
        regmatch_t san_piece = match[1];
        regmatch_t san_move = match[2];

        if (san_piece.rm_eo >= 0 || san_piece.rm_so >= 0)
            piece = san[san_piece.rm_so];

        move.piece = piece;
        move.x = san[san_move.rm_so] - 'a';
        move.y = 7 - (san[san_move.rm_eo - 1] - '1');
    }
    else if (reti == REG_NOMATCH) {
        fprintf(stderr, "no match found\n");
    }
    else {
        regerror(reti, &regex, msgbuf, sizeof(msgbuf));
        fprintf(stderr, "regex match failed: %s\n", msgbuf);
        //exit(reti);
    }

    regfree(&regex);
    return move;
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
            moves[valid].piece = board[orig_x][orig_y];
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
    printf("board:\n");
    print_board();
    struct move * moves;
    int l = m_moves(&moves, 4, 7, k_mvects, sizeof(k_mvects)/sizeof(k_mvects[0]));
    print_moves(moves, l);
    free(moves);

    printf("algebraic notation:\n");
    struct move san_test_moves[2];
    san_test_moves[0] = san_to_move("Nf3");
    san_test_moves[1] = san_to_move("c5");
    printf("\tNf3 -> ");
    print_moves(san_test_moves, 1);
    printf("\tc5 -> ");
    print_moves(san_test_moves+1, 1);

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
