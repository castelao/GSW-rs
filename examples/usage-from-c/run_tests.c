#include <stdio.h>
#include <gswteos-10.h>

int main() {
    printf("%.15f", gsw_specvol(1., 1., 1.));

    return 0;
}
