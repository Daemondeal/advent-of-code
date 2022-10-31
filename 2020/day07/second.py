INPUT_FILE = "./puzzle.txt"

def bags_contained_in_bag(rules, current_bag):
    bags_contained = rules[current_bag]

    total = 0


    for amount, bag in bags_contained:
        total += (bags_contained_in_bag(rules, bag) + 1) * amount

    return total


def main():
    with open(INPUT_FILE, "r") as infile:
        rules = {}

        for line in infile:
            tokens = line.strip().split(" ")

            name = tokens[0] + " " + tokens[1]

            if tokens[4] == "no":
                rules[name] = []
            else:
                bags = []

                for i in range(4, len(tokens), 4):
                    amount = int(tokens[i])

                    bag_name = tokens[i + 1] + " " + tokens[i + 2]
                    bags.append((amount, bag_name))
                
                rules[name] = bags
    
    print(bags_contained_in_bag(rules, "shiny gold"))
    


if __name__ == "__main__":
    main()