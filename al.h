
#ifndef AL_H
#define AL_H

#include <sys/types.h>

#define     AL_INIT_LEN     8

struct al {
    void * data;    /* start of list */
    size_t sz;      /* size of each element in the list */
    int n;          /* number of elements in the list */
    int max;        /* maximum number of elements before reallocation */
};

struct al * al_new(size_t sz);
void al_free(struct al * al);
void al_add(struct al * al, void * elem);

#endif
