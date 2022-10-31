from typing import TextIO


INPUT_FILE = "./puzzle.txt"

class LoopingMap:
    def __init__(self, file: TextIO):
        self.map = []
        for line in file:
            self.map.append(list(line.strip()))
        
    def get(self, x: int, y: int) -> str:
        return self.map[y][x % self.get_width()]

    def get_width(self) -> int:
        return len(self.map[0])

    def get_height(self) -> int:
        return len(self.map)

def check_tress(tree_map: LoopingMap, right: int, down: int) -> int:
    x = 0
    y = 0

    trees = 0
    while y < tree_map.get_height():
        if tree_map.get(x, y) == "#":
            trees += 1

        y += down
        x += right

    return trees

def main():
    with open(INPUT_FILE, "r", encoding="utf8") as infile:
        tree_map = LoopingMap(infile)


    checks = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ]

    product = 1

    for right, down in checks:
        product *= check_tress(tree_map, right, down)

    print(product)


    # for y in range(tree_map.get_height()):
    #     for x in range(tree_map.get_width() * 2):
    #         print(tree_map.get(x, y), end="")
    #     print()

if __name__ == "__main__":
    main()