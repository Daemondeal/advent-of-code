from math import ceil

INPUT_FILE = "./puzzle.txt"

# Solves an equation of the form s + x = k * b, where s, x, b are positive integers and k is the smallest possible positive integer
def solve(s, b):
    k = int(ceil(float(s) / b))
    return k * b - s


def main():
    with open(INPUT_FILE, "r") as infile:
        arrival_time = int(infile.readline().strip())


        buses = map(lambda x: int(x), filter(lambda x: x != "x", infile.readline().strip().split(",")))
    
    minimum_wait = -1
    min_bus_id = -1

    for bus in buses:
        wait = solve(arrival_time, bus)
        if minimum_wait < 0 or minimum_wait > wait:
            minimum_wait = wait
            min_bus_id = bus
    
    print(min_bus_id, minimum_wait, minimum_wait * min_bus_id)


if __name__ == "__main__":
    main()