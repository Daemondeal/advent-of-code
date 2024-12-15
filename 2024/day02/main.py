import sys


def main():
    if len(sys.argv) != 2:
        print("USAGE: python main.py [input_file]")
        return

    with open(sys.argv[1], "r") as infile:
        input_content = infile.read()

    print(f"A: {solve_a(input_content)}")
    print(f"B: {solve_b(input_content)}")


def process_input(input: str) -> list[list[int]]:
    reports = []
    for line in input.split("\n"):
        line = line.strip()
        if line == "":
            continue

        levels = [int(x) for x in line.split(" ")]
        if len(levels) > 0:
            reports.append(levels)

    return reports


def solve_a(input) -> int:
    reports = process_input(input)
    total_safe = sum([1 if is_safe(report) else 0 for report in reports])

    return total_safe


def is_safe(report):
    if len(report) <= 1:
        return True

    decreasing = (report[0] - report[1]) > 0
    last = report[0]

    for level in report[1:]:
        monotone = ((last - level) > 0) == decreasing
        jump_small = 1 <= abs(level - last) <= 3
        last = level

        if not monotone or not jump_small:
            return False

    return True


def solve_b(input) -> int:
    reports = process_input(input)

    total_safe = 0

    for report in reports:
        for idx, _ in enumerate(report):
            levels = report.copy()
            del levels[idx]
            if is_safe(levels):
                total_safe += 1
                break

    return total_safe


if __name__ == "__main__":
    main()
