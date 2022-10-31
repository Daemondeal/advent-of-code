INPUT_FILE = "./puzzle.txt"

def main():
    valid_passwords = 0

    with open(INPUT_FILE, "r", encoding="utf8") as infile:
        for line in infile:
            if line != "":
                bounds, letter, password = line.strip().replace(":", "").split(" ")
                start, end = map(lambda x: int(x) - 1, bounds.split('-'))

                valid_count = 0

                if len(password) > start and password[start] == letter:
                    valid_count += 1 
                if len(password) > end and password[end] == letter:
                    valid_count += 1
                
                if valid_count == 1:
                    valid_passwords += 1

    print(valid_passwords)


if __name__ == "__main__":
    main()