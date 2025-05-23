#include <stdint.h>
#include <stdlib.h>

// See comments in build_native_lib()
#define EXPORT __attribute__((visibility("default")))

EXPORT char* allocate() {
    return malloc(1);
}