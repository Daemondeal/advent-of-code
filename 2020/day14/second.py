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


def get_all_possibilities(number, mask):
    exp = len(mask["floating"])
    possibilities = []

    for i in range(2 ** exp):
        current = []

        for e in range(exp):
            current.append(1 if (2**e) & i >= 1 else 0)

        normal_mask = []

        for exponent in mask["ones"]:
            normal_mask.append((exponent, 1))
        
        for i, exponent in enumerate(mask["floating"]):
            normal_mask.append((exponent, current[i]))

        possibilities.append(apply_mask(number, normal_mask))
    
    return possibilities

def main():
    with open(INPUT_FILE, "r") as infile:
        mask = {
            "ones": [],
            "floating": []
        }
        
        memory = {}

        for line in infile:
            if line.startswith("mask"):
                raw_mask = line.split("=")[1].strip()

                mask = {
                    "ones": [],
                    "floating": []
                }

                for i, val in enumerate(raw_mask):
                    exponent = len(raw_mask) - i - 1

                    if val == "1":
                        mask["ones"].append(exponent)
                    elif val == "X":
                        mask["floating"].append(exponent)

            else:
                position = int(line.split("[")[1].split("]")[0])
                number = int(line.split("=")[1].strip())

                for address in get_all_possibilities(position, mask):
                    memory[address] = number

    print(sum(memory.values()))

if __name__ == "__main__":
    main()