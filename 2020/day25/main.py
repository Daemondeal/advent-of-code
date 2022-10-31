def main():
    print(solve_1())

MODULUS = 20201227

def solve_1():
    pub_a = 8252394
    pub_b = 6269621

    a = bruteforce(pub_a)
    b = bruteforce(pub_b)

    return (a, b)

    # print(transform(7, 8))
    # print(modular_pow(7, 8, MODULUS))





    # enc_squared = (pub_a * pub_b) % MODULUS

    # enc = modular_pow(enc_squared, (MODULUS + 1) // 4, MODULUS)
    # return enc


MAX_BRUTE = 100_000_000

def bruteforce(public):
    print("Bruteforcing ", public)
    for i in range(MAX_BRUTE):
        if i % (MAX_BRUTE // 100) == 0:
            print(i)
        if modular_pow(7, i, MODULUS) == public:
            return i
    
    return -1

def transform(subject_number, loop_size):
    value = 1
    for _ in range(loop_size):
        value *= subject_number
        value %= 20201227
    return value

def modular_pow(base, exponent, modulus):
    if modulus == 1:
        return 0
    c = 1

    for e_prime in range(0, exponent):
        c = (c * base) % modulus

    return c

def handshake(card_loop_size, door_loop_size):
    card_public = transform(7, card_loop_size)
    door_public = transform(7, door_loop_size)


if __name__ == "__main__":
    main()