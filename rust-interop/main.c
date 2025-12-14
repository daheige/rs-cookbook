#include <stdio.h>
#include "rust_interop.h"

int main() {
    int32_t sum = add(5, 3);
    printf("5 + 3 = %d\n", sum);

    return 0;
}