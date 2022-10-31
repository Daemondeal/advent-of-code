from copy import deepcopy

INPUT_FILE = "./puzzle.txt"

def is_ticket_valid(ticket, rules):
    for entry in ticket:
        valid = False

        for rule in rules:
            for r in rules[rule]:
                if entry in r:
                    valid = True
                    break

        if not valid:
            return False
    
    return True


def check_rule(rule, n):
    for r in rule:
        if n in r:
            return True
    
    return False

def main():
    with open(INPUT_FILE, "r") as infile:
        text = infile.read()

    rules_text, other = text.split("your ticket")

    rules = {}

    for line in rules_text.strip().split("\n"):

        name = line.split(":")[0].strip()

        raw_rules = line.split(":")[1].strip().split(" or ")
        rules[name] = []
        for r in raw_rules:
            start, end = r.split("-")
            rules[name].append(range(int(start), int(end) + 1))

    my_ticket = list(map(lambda x: int(x), other.split(":")[1].split("nearby")[0].strip().split(",")))

    other_tickets_text = other.split(":")[2]

    nearby_tickets = []

    for line in other_tickets_text.strip().split("\n"):
        ticket = list(map(lambda x: int(x), line.split(",")))
        if is_ticket_valid(ticket, rules):
            nearby_tickets.append(ticket)

    possibilities = []

    for _ in my_ticket:
        possibilities.append(set(rules.keys()))

    for ticket in nearby_tickets:
        to_remove = []

        for i, field in enumerate(ticket):
            
            for rule in possibilities[i]:
                if not check_rule(rules[rule], field):
                    to_remove.append((i, rule))
        
        for i, rule in to_remove:
            possibilities[i].remove(rule)


    changes = 1
    while changes > 0:
        changes = 0

        only_ones = []
        for p in possibilities:
            if len(p) == 1:
                only_ones.append(next(iter(p)))
        
        for p in possibilities:
            for o in only_ones:
                if len(p) != 1 and o in p:
                    p.remove(o)
                    changes += 1
                
    if sum(map(lambda x: len(x), possibilities)) == len(possibilities):
        print("Success!")
    else:
        print("Not enough constraints...")
        print(possibilities)
        return


    correct_fields = list(map(lambda x: next(iter(x)), possibilities))

    parsed_ticket = {}

    for i, field in enumerate(my_ticket):
        parsed_ticket[correct_fields[i]] = field


    prod = 1
    for field in parsed_ticket:
        if field.startswith("departure"):
            prod *= parsed_ticket[field]

    print(parsed_ticket)
    print(prod)

if __name__ == "__main__":
    main()