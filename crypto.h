#ifndef __crypto
#define __crypto

#include <stdbool.h>

typedef unsigned char uc;
typedef unsigned char* uc_p;

// All below assume a > b

uc multiply(uc a, uc b);

// Get the highest bit position of num
uc highest_bit(uc num);

// Return a // b
bool divide(uc_p a, uc_p b, uc_p quotient, uc_p remainder);

void gf28_mod(short* num);

void swap(short* n1, short* n2);

short inverse_gf28(short a);

// Return gcd(a, b)
// a * s + b * t = gcd(a, b)
short egcd(short* a, short* b, short* s, short* t);

#endif
