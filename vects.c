
#include "fry.h"

struct d_vect
q_dvects[] = {
    { .dx = -1, .dy =  0 }, /* lateral */
    { .dx =  1, .dy =  0 },
    { .dx =  0, .dy = -1 },
    { .dx =  0, .dy =  1 },
    { .dx = -1, .dy = -1 }, /* diagonal */
    { .dx =  1, .dy =  1 },
    { .dx = -1, .dy =  1 },
    { .dx =  1, .dy = -1 }
};

struct d_vect
r_dvects[] = {
    { .dx = -1, .dy =  0 },
    { .dx =  1, .dy =  0 },
    { .dx =  0, .dy = -1 },
    { .dx =  0, .dy =  1 }
};

struct d_vect
b_vects[] = {
    { .dx = -1, .dy = -1 },
    { .dx =  1, .dy =  1 },
    { .dx = -1, .dy =  1 },
    { .dx =  1, .dy = -1 }
};

struct m_vect
wp_mvects[] = {
    { .dx = -1, .dy = -1 },
    { .dx = -0, .dy = -1 },
    { .dx =  1, .dy = -1 }
};

struct m_vect
bp_mvects[] = {
    { .dx = -1, .dy =  1 },
    { .dx = -0, .dy =  1 },
    { .dx =  1, .dy =  1 }
};

struct m_vect
n_mvects[] = {
    { .dx = -2, .dy = -1 },
    { .dx = -2, .dy =  1 },
    { .dx = -1, .dy = -2 },
    { .dx = -1, .dy =  2 },
    { .dx =  1, .dy = -2 },
    { .dx =  1, .dy =  2 },
    { .dx =  2, .dy = -1 },
    { .dx =  2, .dy =  1 }
};

struct m_vect
k_mvects[] = {
    { .dx = -1, .dy = -1 },
    { .dx =  0, .dy = -1 },
    { .dx =  1, .dy = -1 },
    { .dx = -1, .dy =  0 },
    { .dx =  1, .dy =  0 },
    { .dx = -1, .dy =  1 },
    { .dx =  0, .dy =  1 },
    { .dx =  1, .dy =  1 }
};
