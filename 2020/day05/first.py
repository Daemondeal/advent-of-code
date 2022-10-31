INPUT_FILE = "./puzzle.txt"


def get_int(binary: str, one_char: str) -> int:
    total = 0

    for i, ch in enumerate(binary):
        if ch == one_char:
            total += 2 ** (len(binary) - i - 1)
    
    return total

def get_seat_id(code: str) -> int:
    row = get_int(code[:7], "B")
    col = get_int(code[7:], "R")

    return row * 8 + col


def main():
    
    with open(INPUT_FILE, "r") as infile:
        lines = infile.readlines()

        highest = get_seat_id(lines[0].strip())

        for line in lines[1:]:
            sid = get_seat_id(line.strip())

            if sid > highest:
                highest = sid

        print(highest)


if __name__ == "__main__":
    main()