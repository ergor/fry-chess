
#ifndef SAN_H
#define SAN_H

#include "fry.h"

/**
 * Regex capture groups:
 * 0: whole match
 * 1: moving piece
 * 2: disambiguation: files
 * 3: disambiguation: ranks
 * 4: capture
 * 5: destination square
 */
#define SAN_REGEX       "([RNBQK])*([a-h])*([1-8])*(x)*([a-h][1-8])"
#define REGEX_GROUP_SZ  6

#define REGEX_NO_MATCH  'X'

#define REGMATCH_ALL(regmatch)         (regmatch[0])
#define REGMATCH_PIECE(regmatch)       (regmatch[1])
#define REGMATCH_FROM_FILE(regmatch)   (regmatch[2])
#define REGMATCH_FROM_RANK(regmatch)   (regmatch[3])
#define REGMATCH_CAPTURE(regmatch)     (regmatch[4])
#define REGMATCH_LAND_SQ(regmatch)     (regmatch[5])

struct move {
    char piece;             /* the ASCII representation of the piece */
    int from_file;          /* */
    int from_rank;          /* */
    struct vect landing_sq; /* */
    bool capture;           /* */
};

struct move san_to_move(char * san_string, int ply);
void print_move(struct move move);

#endif
