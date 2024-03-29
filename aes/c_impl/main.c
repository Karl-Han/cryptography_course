#include <assert.h>
#include <stdbool.h>
#include <string.h>
#include "crypto.h"

short sbox[] = {
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b,
    0xfe, 0xd7, 0xab, 0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0,
    0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26,
    0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
    0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0,
    0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed,
    0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f,
    0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
    0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec,
    0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14,
    0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c,
    0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
    0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f,
    0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e,
    0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1, 0xf8, 0x98, 0x11,
    0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f,
    0xb0, 0x54, 0xbb, 0x16};

bool test_new_modulo(short xor_num) {
    bool arr[0x100];
    memset(arr, 0, sizeof(bool) * 0x100);
    for (short h = 1; h < 0x100; h++) {
        for (short l = 1; l < 0x100; l++) {
            // for some h, h * l is one to one
            short res = multiply(h, l, xor_num);
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
    return true;
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
        assert(multiply(ins, i, 0x1b) == 1);
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
    highest_bit_test();
    divide_test();
    egcd_test();
    s_box_test();

    //printf("%d", inverse_gf28(1));
    short q, r;
    divide(0x11b, 2, &q, &r);
    printf("q = %d, r = %d\n", q, r);
    printf("%x", multiply(0x11b, 2, 0x1b) ^ r);
    assert(test_new_modulo(0x1b) == true);
    assert(test_new_modulo(0x1d) == true);
    assert(test_new_modulo(0x1c) == false);
    return 0;
}
