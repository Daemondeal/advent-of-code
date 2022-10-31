import sys

def main():
    if len(sys.argv) != 2:
        print("USAGE: python main.py [input_file]")
        return

    with open(sys.argv[1], "r") as infile:
        input_content = infile.read()

    print(f"A: {solve_a(input_content)}")
    print(f"B: {solve_b(input_content)}")


def process_input(input: str) -> list[int]:
    pass


def solve_a(input) -> int:
    return 0

def solve_b(input) -> int:
    return 0

if __name__ == "__main__":
    main()
