INPUT_FILE = "./puzzle.txt"

def evaluate_expression(expr):
    paren_level = 0

    current_op = "+"
    val = 0

    captured = []

    for ch in expr:
        if paren_level > 0:
            captured.append(ch)

        if ch == "(":
            paren_level += 1
        elif ch == ")":
            paren_level -= 1

            if paren_level == 0:
                captured.pop()

                result = evaluate_expression(captured)

                # print(f"doing {val} {current_op} {result}")

                if current_op == "+":
                    val += result
                else:
                    val *= result


                captured.clear()

        elif paren_level == 0:
            if ch.isnumeric():
                if current_op == "+":
                    val += int(ch)
                elif current_op == "*":
                    val *= int(ch)
            elif ch == "+":
                current_op = "+"
            elif ch == "*":
                current_op = "*"

    return val


def main():
    total = 0
    with open(INPUT_FILE, "r") as infile:
        for line in infile:
            total += evaluate_expression(line.strip())
            # print(line.strip(), "=", evaluate_expression(line.strip()))

    print(total)

if __name__ == "__main__":
    main()