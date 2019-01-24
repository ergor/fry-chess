
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
    al->sz = sz;
    al->n = 0;
    al->data = malloc(LEN(al, al->max));//al->max * sz);

    return al;
}

void
al_free(struct al * al)
{
    free(al->data);
    free(al);
}

void
al_free_lstlst(struct al * als)
{
    //struct al * als_array = GET_LISTS(als);
    for (int i = 0; i < als->n; i++) {
        al_free(al_get(als, i));
    }
}

/**
 * Copies bytes from element into next position in array list.
 * Args:
 *         al: the array list to add the element to
 *  elem_addr: the address of the element to add
 */
void
al_add(struct al * al, void * elem_addr)
{
    if (al->n == al->max) {
        void * old = al->data;
        size_t old_len = al->max;

        al->max *= 2;
        al->data = malloc(al->max * al->sz);

        memcpy(al->data, old, LEN(al, old_len));
        
        free(old);
    }

    memcpy(al->data + LEN(al, al->n), elem_addr, al->sz);
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

    return al->data + LEN(al, index);
}
