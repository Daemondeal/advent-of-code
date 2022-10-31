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

def main():
    with open(INPUT_FILE, "r", encoding="utf8") as infile:
        tree_map = LoopingMap(infile)

    right = 3
    down = 1

    x = 0
    y = 0

    trees = 0
    while y < tree_map.get_height():
        if tree_map.get(x, y) == "#":
            trees += 1

        y += down
        x += right

    print(trees)



    # for y in range(tree_map.get_height()):
    #     for x in range(tree_map.get_width() * 2):
    #         print(tree_map.get(x, y), end="")
    #     print()

if __name__ == "__main__":
    main()