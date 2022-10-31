INPUT_FILE = "./puzzle.txt"


def run(program, flipped_index = -1):
    accumulator = 0
    index_visited = set()
    pc = 0

    while pc not in index_visited and pc < len(program):
        index_visited.add(pc)

        op, arg = program[pc]
        if pc == flipped_index:
            if op == "nop":
                op = "jmp"
            else:
                op = "nop"

        if op == "nop": # do nothing
            pass
        elif op == "acc":
            accumulator += arg
        elif op == "jmp":
            pc += arg
            continue

        pc += 1

    return (pc < len(program), accumulator) # {"infinite-loop": pc <= len(program), "accumulator": accumulator}


def main():
    with open(INPUT_FILE, "r") as infile:
        program = []

        for line in infile:
            operation, argument = line.strip().split(" ")
            program.append((operation, int(argument)))
    

    for i, instruction in enumerate(program):
        if instruction[0] in ["jmp", "nop"]:
            is_infinite_loop, accumulator = run(program, i)
            if not is_infinite_loop:
                print(i, accumulator)


if __name__ == "__main__":
    main()