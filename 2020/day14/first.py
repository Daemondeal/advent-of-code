INPUT_FILE = "./puzzle.txt"


def apply_mask(number, mask):
    full_mask = 2 ** 36 - 1
    empty_mask = 0

    for exp, val in mask:
        if val == 0:
            number &= full_mask - 2 ** exp
        elif val == 1:
            number |= empty_mask + 2 ** exp

    return number

def main():
    with open(INPUT_FILE, "r") as infile:
        mask = []
        
        memory = {}

        for line in infile:
            if line.startswith("mask"):
                raw_mask = line.split("=")[1].strip()
                mask.clear()


                for i, val in enumerate(raw_mask):
                    if val != "X":
                        exponent = len(raw_mask) - i - 1
                        mask.append((exponent, int(val)))

            else:
                position = int(line.split("[")[1].split("]")[0])
                number = int(line.split("=")[1].strip())
                masked_number = apply_mask(number, mask)

                memory[position] = masked_number

        print(sum(memory.values()))


if __name__ == "__main__":
    main()