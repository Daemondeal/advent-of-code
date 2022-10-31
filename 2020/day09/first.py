INPUT_FILE = "./puzzle.txt"
PREAMBLE_LENGTH = 25

def is_sum(preamble, num):
    for n in preamble:
        if (num - n) in preamble and (num - n) != n:
            return True
    
    return False

def main():
    preamble = []

    with open(INPUT_FILE, "r") as infile:
        file_lines = infile.readlines()

        for line in file_lines[:PREAMBLE_LENGTH]:
            preamble.append(int(line.strip()))
        
        for line in file_lines[PREAMBLE_LENGTH:]:
            num = int(line.strip())
            if not is_sum(preamble, num):
                print(num)
                break
            
            preamble.pop(0)
            preamble.append(num)


if __name__ == "__main__":
    main()