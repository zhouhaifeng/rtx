#ifndef ERRNO_H
#define ERRNO_H

#include "../errno.h"

#ifdef __GNUC__
__attribute__((const))
#endif
int *___errno_location(void);

#undef errno
#define errno (*___errno_location())

#endif
