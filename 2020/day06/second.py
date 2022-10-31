INPUT_FILE = "./test.txt"

def main():
    with open(INPUT_FILE, "r") as infile:
        groups = infile.read().strip().split("\n\n")

        total = 0

        for group in groups:
            split_group = group.split('\n')

            total_set = set(split_group[0].strip())

            for person in split_group[1:]:
                total_set = total_set.intersection(set(person.strip()))

            total += len(total_set)
    
    print(total)

if __name__ == "__main__":
    main()