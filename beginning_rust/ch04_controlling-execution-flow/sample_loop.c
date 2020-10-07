#include <stdio.h>
int main() {
    int limit = 10;
    for (int i = 1; i < limit; i++) {
        limit -= 1;
        printf("%d", i);
    }
    printf(":%d", limit);
}

// result: 12345:5
// different to Rust, as limits is decremented, the loop condition is no more satisfied after the 5 iterations.