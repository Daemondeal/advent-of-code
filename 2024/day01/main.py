import sys


def main():
    if len(sys.argv) != 2:
        print("USAGE: python main.py [input_file]")
        return

    with open(sys.argv[1], "r") as infile:
        input_content = infile.read()

    print(f"A: {solve_a(input_content)}")
    print(f"B: {solve_b(input_content)}")


def process_input(input: str) -> tuple[list[int], list[int]]:
    left = []
    right = []
    for line in input.split("\n"):
        if line.strip() == "":
            break

        first, second = line.split("  ")
        left.append(int(first))
        right.append(int(second))

    return left, right


def solve_a(input) -> int:
    left, right = process_input(input)
    left.sort()
    right.sort()

    dist = 0
    for l, r in zip(left, right):
        dist += abs(l - r)

    return dist


def solve_b(input) -> int:
    left, right = process_input(input)

    result = 0
    for num in left:
        count = 0
        for n2 in right:
            if n2 == num:
                count += 1
        result += count * num

    return result


if __name__ == "__main__":
    main()
