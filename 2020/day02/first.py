INPUT_FILE = "./puzzle.txt"

def main():
    valid_passwords = 0

    with open(INPUT_FILE, "r", encoding="utf8") as infile:
        for line in infile:
            if line != "":
                bounds, letter, password = line.strip().replace(":", "").split(" ")
                min_letters, max_letters = map(lambda x: int(x), bounds.split('-'))

                count = password.count(letter)

                if min_letters <= count <= max_letters:
                    valid_passwords += 1

    print(valid_passwords)


if __name__ == "__main__":
    main()