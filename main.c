#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include "crypto.h"

uint8_t gmul(uint8_t a, uint8_t b) {
    uint8_t p = 0; /* the prodshortt of the multiplication */
    while (a && b) {
        if (b & 1)  /* if b is odd, then add the corresponding a to p (final
                       prodshortt = sum of all a's corresponding to odd b's) */
            p ^= a; /* since we're in GF(2^m), addition is an XOR */

        if (a & 0x80) /* GF modulo: if a >= 128, then it will overflow when
                         shifted left, so redshorte */
            a = (a << 1) ^ 0x11b; /* XOR with the primitive polynomial x^8 + x^4
                                     + x^3 + x + 1 (0b1_0001_1011) – you can
                                     change it but it must be irredshortible */
        else
            a <<= 1; /* equivalent to a*2 */
        b >>= 1;     /* equivalent to b // 2 */
    }
    return p;
}

void multiply_test() {
    for (short i = 2; i != 0; i++) {
        for (short j = 1; j != 0; j++) {
            assert(gmul(i, j) == multiply(i, j));
        }
    }
}

void highest_bit_test() { assert(highest_bit(0x80) == 8); }

void divide_test() {
    short a = 127;
    short b = 23;
    short q, r;
    bool bl = divide(&a, &b, &q, &r);
    assert(bl);
    assert(q == 6);
    assert(r == 13);
}

void egcd_test() {
    for (short i = 1; i < 256; i++) {
        short ins = inverse_gf28(i);
        // printf("i = %x, and inverse is %x\n", i, ins);
        assert(multiply(ins, i) == 1);
    }
}

int main() {
    highest_bit_test();
    divide_test();
    egcd_test();
    return 0;
}
