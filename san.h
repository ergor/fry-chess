
#ifndef SAN_H
#define SAN_H

#include "fry.h"

struct san_move {
    char piece;
    struct vect dest;
};

struct san_move san_to_move(char * san, int move_num);

#endif
