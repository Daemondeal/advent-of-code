INPUT_FILE = "./puzzle.txt"


def can_contain_target(rules, current_rule, target="shiny gold"):
    if current_rule == target:
        return False

    if len(rules[current_rule]) == 0:
        return False
    
    if target in rules[current_rule]:
        return True

    for rule in rules[current_rule]:
        if can_contain_target(rules, rule, target):
            return True
    
    return False


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
                    amount = int(tokens[i]) # Not needed

                    bag_name = tokens[i + 1] + " " + tokens[i + 2]
                    bags.append(bag_name)
                
                rules[name] = bags
 
    possible_containers = 0
    for rule in rules:
        if can_contain_target(rules, rule, "shiny gold"):
            possible_containers += 1

    print(possible_containers)


if __name__ == "__main__":
    main()