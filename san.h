
#ifndef SAN_H
#define SAN_H

#include "fry.h"

struct san_move {
    int piece;
    struct pos move;
};

struct san_move san_to_move(char * san);

#endif
