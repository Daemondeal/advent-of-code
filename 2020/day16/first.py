INPUT_FILE = "./puzzle.txt"

def in_ranges(number, ranges):
    for r in ranges:
        if number in r:
            return True
    
    return False

def main():
    with open(INPUT_FILE, "r") as infile:
        text = infile.read()

    rules_text, other = text.split("your ticket")

    rules = []

    for line in rules_text.strip().split("\n"):
        raw_rules = line.split(":")[1].strip().split(" or ")
        for r in raw_rules:
            start, end = r.split("-")
            rules.append(range(int(start), int(end) + 1))


    other_tickets_text = other.split(":")[2]
    error_rate = 0
    
    for line in other_tickets_text.strip().split("\n"):
        ticket = map(lambda x: int(x), line.split(","))
        for n in ticket:
            if not in_ranges(n, rules):
                error_rate += n
    
    print(error_rate)


if __name__ == "__main__":
    main()