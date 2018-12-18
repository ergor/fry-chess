
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
b_dvects[] = {
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

void * vects[128];

void init_vects()
{
    vects['p'] = bp_mvects;
    vects['P'] = wp_mvects;
    vects['k'] = k_mvects;
    vects['K'] = k_mvects;
    vects['q'] = q_dvects;
    vects['Q'] = q_dvects;
    vects['r'] = r_dvects;
    vects['R'] = r_dvects;
    vects['b'] = b_dvects;
    vects['B'] = b_dvects;
    vects['n'] = n_mvects;
    vects['N'] = n_mvects;
}
