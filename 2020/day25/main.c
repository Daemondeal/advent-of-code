#include <stdio.h>

#define MODULUS 20201227

typedef unsigned long long u64;


u64 transform(u64 subject_number, u64 loop_size) {
    u64 i;
    u64 value = 1;

    for (i = 0; i < loop_size; i++) {
        value *= subject_number;
        value %= MODULUS;
    }

    return value;
}

u64 bruteforce(u64 target) {
    u64 loops = 0;
    u64 value = 1;

    printf("Value: %d\n", value);
    printf("Target: %d\n", target);
    while (value != target) {
        value *= 7;
        value %= MODULUS;
        loops++;
    }

    return loops;
}


int main() {
    u64 pub_a = 8252394;
    u64 pub_b = 6269621;
    printf("Bruteforcing...\n");
    u64 sec_a = bruteforce(pub_a);

    printf("sec_a: %llu\n");
    printf("enc: %llu\n", transform(pub_b, sec_a));

    return 0;
}