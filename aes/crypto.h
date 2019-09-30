#ifndef __crypto
#define __crypto

#include <stdbool.h>

// All below assume a > b

short gf28_mod(short* num);

void swap(short* n1, short* n2);

short multiply(short a, short b);

// Get the highest bit position of num
short highest_bit(short num);

// Return a // b
bool divide(short a, short b, short* quotient, short* remainder);

short inverse_gf28(short a);

// Return gcd(a, b)
// a * s + b * t = gcd(a, b)
short egcd(short* a, short* b, short* s, short* t);

short s_box(short num);

#endif
