from math import ceil

INPUT_FILE = "./puzzle.txt"


def extended_gcd(a, b):
    if a == 0:
        return b, 0, 1
    else:
        gcd, x, y = extended_gcd(b % a, a)
        return gcd, y - (b // a) * x, x

# Resolves a a^-1 = 1 (mod m) for a^-1
def modular_inverse(a, m):
    gcd, x, _ = extended_gcd(a, m)
    if gcd != 1:
        raise Exception("Modular inverse does not exist")

    return x % m

def main():
    with open(INPUT_FILE, "r") as infile:
        infile.readline() # ignored

        raw_buses = []
        for i, b in enumerate(infile.readline().strip().split(",")):
            if b != "x":
                raw_buses.append((int(b), i))
    
    # equations x = b (mod i)

    buses = []

    for m, i in raw_buses:
        buses.append((m, (m - i) % m))


    M = 1
    for b, _ in buses:
        M *= b

    x = 0

    print(buses, M)

    for m, i in buses:
        print(f"x = {i} mod {m}")

        n = modular_inverse(M // m, m)
        x += i * n * M // m

    print()
    print("Computed:")

    for m, i in buses:
        print(f"x = {x % m} mod {m}")


    print(x % M)


if __name__ == "__main__":
    main()