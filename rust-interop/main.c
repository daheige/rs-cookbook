#include <stdio.h>
#include "rust_interop.h"

int main() {
    // 通过ffi方式调用rust定义的函数
    int32_t sum = add(5, 3);
    printf("5 + 3 = %d\n", sum);

    say_hello();

    call_from_rust();

    // 调用hello函数
    hello("daheige");

    return 0;
}