
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <regex.h>
#include <sys/types.h>

#include "fry.h"
#include "san.h"

/**
 * Args:
 *  san_string:
 *         ply: current half-turn of the game
 */
struct move
san_to_move(char * san_string, int ply)
{
    char msgbuf[1024];

    bool black_turn = ply & 1;

    char piece = REGEX_NO_MATCH;
    struct vect landing_sq = { .x = -1, .y = -1 };

    regex_t regex;
    regmatch_t match[REGEX_GROUP_SZ];

    int reti = regcomp(&regex, SAN_REGEX, REG_EXTENDED);
    if (reti) {
        fprintf(stderr, "failed to compile regex\n");
        exit(reti);
    }

    reti = regexec(&regex, san_string, REGEX_GROUP_SZ, match, 0);
    if (reti == 0) {
        piece = 'P';

        if (REGMATCH_PIECE(match).rm_eo >= 0
            || REGMATCH_PIECE(match).rm_so >= 0)
        {
            piece = san_string[REGMATCH_PIECE(match).rm_so];
        }

        if (black_turn)
            piece += 0x20; // to_lower for black move

        landing_sq.x = san_string[REGMATCH_LAND_SQ(match).rm_so] - 'a';
        landing_sq.y = 7 - (san_string[REGMATCH_LAND_SQ(match).rm_eo -1] - '1');
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
    struct move move = { .piece = piece, .landing_sq = landing_sq };

    return move;
}

void
print_move(struct move move)
{
    printf("move struct:\n  piece: %c\n  from_file: %d\n  from_rank: %d\n",
            move.piece,
            move.from_file,
            move.from_rank);
    printf("  landing_sq:\n    x: %d\n    y: %d\n  capture: %d\n\n",
            move.landing_sq.x,
            move.landing_sq.y,
            move.capture);
}
