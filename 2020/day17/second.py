INPUT_FILE = "./puzzle.txt"

class ConwayGrid:
    def __init__(self, board):
        self.active_states = set()

        for x, row in enumerate(board):
            for y, cell in enumerate(row):
                if cell == "#":
                    self.active_states.add((x, y, 0, 0))

    def is_active(self, x, y, z, w):
        return (x, y, z, w) in self.active_states

    def do_step(self):
        adjacencies = {}

        steps = []

        for i in range(-1, 2):
            for j in range(-1, 2):
                for k in range(-1, 2):
                    for l in range(-1, 2):
                        if i != 0 or j != 0 or k != 0 or l != 0:
                            steps.append((i, j, k, l))

        for x, y, z, w in self.active_states:
            if (x, y, z, w) not in adjacencies:
                adjacencies[(x, y, z, w)] = 0

            for dx, dy, dz, dw in steps:
                cell = (x + dx, y + dy, z + dz, w + dw)

                if cell not in adjacencies:
                    adjacencies[cell] = 0
                
                adjacencies[cell] += 1


        for cell in adjacencies:
            neighbours = adjacencies[cell]

            if cell in self.active_states:
                if neighbours not in [2, 3]:
                    self.active_states.remove(cell)
            else:
                if neighbours == 3:
                    self.active_states.add(cell)


    def get_active_cells(self):
        return len(self.active_states)

def main():
    with open(INPUT_FILE, "r") as infile:

        grid = ConwayGrid(infile.readlines())

    for _ in range(6):
        grid.do_step()

    print(grid.get_active_cells())

if __name__ == "__main__":
    main()