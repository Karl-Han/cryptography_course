#include "crypto.h"
#include <assert.h>

// typedef unsigned char short;
// typedef unsigned char* short*;

short gf28_mod(short* num) {
    *num = (*num + 283) % 283;
    return *num;
}

void swap(short* n1, short* n2) {
    short temp = *n1;
    *n1 = *n2;
    *n2 = temp;
}

short multiply(short a, short b, short xor_num) {
    short res = 0;
    while (a && b) {
        if (b & 1) {
            res ^= a;
        }
        // Test if a will overflow when *2
        if (a & 0x80) {
            a = (a << 1) ^ xor_num;
        } else {
            a <<= 1;
        }
        b >>= 1;
    }
    return res % 256;
}

short highest_bit(short num) {
    short counter = 0;
    while (num) {
        counter++;
        num >>= 1;
    }
    return counter;
}

// All below assume a > b

// Return whether it is right
bool divide(short a, short b, short* quotient, short* remainder) {
    short t = a;
    short q;
    q = 0;
    short b_bits = highest_bit(b);
    short s = (1 << (b_bits - 1)) - 1;
    short hb = highest_bit(a);
    while (hb >= b_bits) {
        hb = highest_bit(a);
        short b_shift = b << (hb - b_bits);
        a ^= b_shift;
        q |= (1 << (hb - b_bits));
        // q <<= (hb - b_bits);
    }
    *quotient = q;
    *remainder = a;
    assert(multiply(*quotient, b, 0x1b) ^ *remainder == 0x1b);
    return t == (multiply(b, *quotient, 0x1b) ^ *remainder);
}

// Return gcd(a, b)
short egcd(short* a, short* b, short* s, short* t) {
    short s1, s2, t1, t2;
    s1 = t2 = 1;
    s2 = t1 = 0;
    short q, r;
    while (*b) {
        divide(*a, *b, &q, &r);
        s1 ^= multiply(t1, q, 0x1b);
        s2 ^= multiply(t2, q, 0x1b);
        *a = *b;
        *b = r;
        swap(&s1, &t1);
        swap(&s2, &t2);
    }
    // gf28_mod(&s1);
    // gf28_mod(&s2);
    *s = s1;
    *t = s2;
    return *a;
}

short inverse_gf28(short a) {
    short b = 0x11b;
    short s, t;
    egcd(&b, &a, &s, &t);
    return t;
}

short s_box(short num) {
    short c = 0x63;
    short temp = 0x0;
    short res = 0x0;
    for (unsigned char i = 0; i < 8; i++) {
        // Get the i th's transform in the last bit
        temp ^= (num >> (i % 8));
        temp ^= (num >> ((i + 4) % 8));
        temp ^= (num >> ((i + 5) % 8));
        temp ^= (num >> ((i + 6) % 8));
        temp ^= (num >> ((i + 7) % 8));
        temp ^= c >> i;
        temp &= 0x01;
        res |= (temp << i);
        temp = 0x0;
    }
    return res;
}
