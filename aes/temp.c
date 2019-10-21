#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

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
    return true;
}

int main() {
    assert(test_new_modulo(0x1b) == true);
    printf("Pass xor 0x1b\n");
    assert(test_new_modulo(0x1d) == true);
    printf("Pass xor 0x1d\n");

    assert(test_new_modulo(0x1c) == false);
    return 0;
}
