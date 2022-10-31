from enum import Enum

INPUT_FILE = "./puzzle.txt"

PRIORITIES = {
    "*": 1,
    "+": 2
}

def tokenize(expr):
    res = []
    
    for ch in expr:
        if ch in "()*+": 
            res.append(("OPERATOR", ch))
        elif ch.isnumeric():
            res.append(("NUMBER", int(ch)))

    return res


def eval_to_parens(stack, should_remove=False):
    total = 0

    cur_op = "+"

    while len(stack) >= 1:    
        cur_type, cur_val = stack.pop()

        if cur_type == "NUMBER":
            if cur_op == "+":
                total += cur_val
            else:
                total *= cur_val
        elif cur_type == "OPERATOR":
            if cur_val in "*+":
                cur_op = cur_val
            elif cur_val == "(":
                if not should_remove:
                    stack.append(("OPERATOR", "("))
                break


    return total

def evaluate_expression(expr):
    stack = []

    tokens = tokenize(expr)


    priority_stack = [-1]

    last_priority = -1

    for token_type, token in tokens:
        if token_type == "NUMBER":
            stack.append((token_type, token))
        elif token_type == "OPERATOR":
            if token == "(":
                stack.append(("OPERATOR", token))
                priority_stack.append(-1)

            elif token == ")":
                ev = eval_to_parens(stack, True)
                priority_stack.pop()
                stack.append(("NUMBER", ev))
                
            elif token in "*+":
                priority = PRIORITIES[token]
                
                if priority_stack.pop() <= priority:
                    stack.append((token_type, token))
                else:
                    ev = eval_to_parens(stack, False)
                    stack.append(("NUMBER", ev))
                    stack.append((token_type, token))

                priority_stack.append(priority)



        # print("Stack: ", end="")
        # for _, t in stack:
        #     print(t, end=" ")
        # print()

    return eval_to_parens(stack, True)


def debug():
    expr = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"

    expr = "2 + (7 * (6 + 9 + 2 * 4)) * 7 * 6"

    print("Doing", expr)
    r = evaluate_expression(expr)
    print("Result:", r)
    print("Should be:", resultadv(expr))



def main():
    total = 0

    with open(INPUT_FILE, "r") as infile:
        for line in infile:
            total += evaluate_expression(line.strip())
            # print(line.strip(), "=", evaluate_expression(line.strip()))

    print(total)

if __name__ == "__main__":
    main()