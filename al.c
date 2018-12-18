
/* al.c - array list implementation */

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <sys/types.h>
#include "al.h"

/**
 * Make new array list where each element is sz bytes in size.
 */
struct al *
al_new(size_t sz)
{
    struct al * al = malloc(sizeof(struct al));

    al->max = AL_INIT_LEN;
    al->data = malloc(al->max * sz);
    al->sz = sz;
    al->n = 0;

    return al;
}

void
al_free(struct al * al)
{
    free(al->data);
    free(al);
}

/**
 * Copies bytes from elem into next position in array list.
 */
void
al_add(struct al * al, void * elem)
{
    if (al->n == al->max) {
        void * old = al->data;
        size_t old_len = al->max;

        al->max *= 2;
        al->data = malloc(al->max * al->sz);

        memcpy(al->data, old, old_len);
        
        free(old);
    }

    memcpy(al->data + (al->n * al->sz), elem, al->sz);
    al->n += 1;
}

void *
al_get(struct al * al, int index)
{
    if (index >= al->n) {
        fprintf(stderr, "[!!] array list: index > length\n");
        return NULL;
    }
    if (index < 0) {
        fprintf(stderr, "[!!] array list: index < 0\n");
        return NULL;
    }

    return al->data + (index * al->sz);
}
