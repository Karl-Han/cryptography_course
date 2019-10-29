#include <assert.h>
#include <stdbool.h>
#include <string.h>
#include "crypto.h"

bool test_new_modulo(short xor_num) {
    bool arr[0x100];
    memset(arr, 0, sizeof(bool) * 0x100);
    for (short h = 1; h < 0x100; h++) {
        for (short l = 1; l < 0x100; l++) {
            // for some h, h * l is one to one
            short res = multiply(h, l, 0x1b);
            if (arr[res] == 0) {
                arr[res] = 1;
            } else {
                printf("ERROR, h = %x, l = %x, res = %x", h, l, res);
                return false;
            }
        }
        for (short l = 1; l < 0x100; l++) {
            if (arr[l] == 0) {
                printf("ERROR, l = %x has no injection.", l);
                return false;
            }
        }
        memset(arr, 0, sizeof(bool) * 0x100);
    }
    printf("Pass multiply_test\n");
}

void highest_bit_test() { assert(highest_bit(0x80) == 8); }

void divide_test() {
    short a = 127;
    short b = 23;
    short q, r;
    bool bl = divide(a, b, &q, &r);
    assert(bl);
    assert(q == 6);
    assert(r == 13);
    printf("Pass divide_test\n");
}

void egcd_test() {
    for (short i = 1; i < 256; i++) {
        short ins = inverse_gf28(i);
        // printf("i = %x, and inverse is %x\n", i, ins);
        assert(multiply(ins, i) == 1);
    }
    printf("Pass egcd_test\n");
}

void s_box_test() {
    for (short i = 0; i < 256; i++) {
        short ins = s_box(inverse_gf28(i));
        // printf("%d's ins = %02x, sbox[i] = %02x\n", i, ins, sbox[i]);
        assert(ins == sbox[i]);
    }
    printf("Pass s_box_test\n");
}

unsigned char sub_byte(unsigned char c) { return s_box(inverse_gf28(c)); }

int main() {
    assert(test_new_modulo(0x1b) == true);
    assert(test_new_modulo(0x1d) == true);

    assert(test_new_modulo(0x1c) == false);
    return 0;
}
