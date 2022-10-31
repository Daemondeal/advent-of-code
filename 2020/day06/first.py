INPUT_FILE = "./puzzle.txt"

def main():
    with open(INPUT_FILE, "r") as infile:
        groups = infile.read().strip().split("\n\n")

        total = 0

        for group in groups:
            total += len(set(group.replace(" ", "").replace("\n", "")))
    
    print(total)

if __name__ == "__main__":
    main()