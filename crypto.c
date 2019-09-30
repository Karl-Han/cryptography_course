#include "crypto.h"

// typedef unsigned char uc;
// typedef unsigned char* uc_p;

void gf28_mod(short* num) { *num = (*num + 256 % 256); }

void swap(short* n1, short* n2) {
    *n1 = *n1 ^ *n2;
    *n2 = *n1 ^ *n2;
    *n1 = *n1 ^ *n2;
}

uc multiply(uc a, uc b) {
    uc res = 0;
    while (a && b) {
        if (b & 1) {
            res ^= a;
        }
        // Test if a will overflow when *2
        if (a & 0x80) {
            a = (a << 1) ^ 0x1b;
        } else {
            a <<= 1;
        }
        b >>= 1;
    }
    return res;
}

uc highest_bit(uc num) {
    uc counter = 0;
    while (num) {
        counter++;
        num >>= 1;
    }
    return counter;
}

// All below assume a > b

// Return whether it is right
bool divide(uc_p a, uc_p b, uc_p quotient, uc_p remainder) {
    uc q, r;
    q = 0;
    r = *a;
    uc b_bits = highest_bit(*b);
    uc s = (1 << (b_bits - 1)) - 1;
    uc hb = highest_bit(r);
    while (hb >= b_bits) {
        hb = highest_bit(r);
        uc b_shift = (*b) << (hb - b_bits);
        r ^= b_shift;
        q |= (1 << (hb - b_bits));
        // q <<= (hb - b_bits);
    }
    *quotient = q;
    *remainder = r;
    return (*a) == (multiply(*b, *quotient) ^ *remainder);
}

short inverse_gf28(short a) {
    short b = 0x11b;
    short s, t;
    if (a > b) {
        egcd(&a, &b, &s, &t);
        return s;
    } else {
        egcd(&b, &a, &s, &t);
        return t;
    }
}

short egcd(short* a, short* b, short* s, short* t) {
    short s1, s2, t1, t2;
    s1 = t2 = 1;
    s2 = t1 = 0;
    short q, r;
    while (*b) {
        divide(a, b, &q, &r);
        s1 ^= multiply(t1, q);
        s2 ^= multiply(t2, q);
        gf28_mod(&s1);
        gf28_mod(&s2);
        *a = *b;
        *b = r;
        swap(&s1, &t1);
        swap(&s2, &t2);
    }
    gf28_mod(&s1);
    gf28_mod(&s2);
    return *a;
}
