
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <regex.h>
#include <sys/types.h>

#include "fry.h"
#include "san.h"

/**
 * Args:
 *  move_num: current move number of this game
 */
struct san_move
san_to_move(char * san, int move_num)
{
    char msgbuf[1024];

    bool black_turn = move_num & 1;

    char piece = 'X';
    struct vect dest = { .x = -1, .y = -1 };
    
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
        regmatch_t r_piece = match[1];
        regmatch_t r_move = match[2];

        piece = 'P';

        if (r_piece.rm_eo >= 0 || r_piece.rm_so >= 0)
            piece = san[r_piece.rm_so];

        dest.x = san[r_move.rm_so] - 'a';
        dest.y = 7 - (san[r_move.rm_eo - 1] - '1');
    }
    else if (reti == REG_NOMATCH) {
        fprintf(stderr, "no match found\n");
    }
    else {
        regerror(reti, &regex, msgbuf, sizeof(msgbuf));
        fprintf(stderr, "regex match failed: %s\n", msgbuf);
        //exit(reti);
    }

    if (black_turn)
        piece += 0x20; // lower case for black pieces

    regfree(&regex);
    struct san_move san_move = { .piece = piece, .dest = dest };

    return san_move;
}
