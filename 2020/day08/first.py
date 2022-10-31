INPUT_FILE = "./puzzle.txt"


def run(program):
    accumulator = 0
    index_visited = set()
    pc = 0

    while pc not in index_visited and pc < len(program):
        index_visited.add(pc)

        op, arg = program[pc]

        if op == "nop": # do nothing
            pass
        elif op == "acc":
            accumulator += arg
        elif op == "jmp":
            pc += arg
            continue

        pc += 1

    return accumulator


def main():
    with open(INPUT_FILE, "r") as infile:
        program = []

        for line in infile:
            operation, argument = line.strip().split(" ")
            program.append((operation, int(argument)))
    

    print(run(program))


if __name__ == "__main__":
    main()