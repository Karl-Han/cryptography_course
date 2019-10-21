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
    return true;
}

int main() {
    assert(test_new_modulo(0x1b) == true);
    assert(test_new_modulo(0x1d) == true);

    assert(test_new_modulo(0x1c) == false);
    return 0;
}
