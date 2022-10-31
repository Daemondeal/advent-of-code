INPUT_FILE = "./puzzle.txt"

class ConwayGrid:
    def __init__(self, board):
        self.active_states = set()

        for x, row in enumerate(board):
            for y, cell in enumerate(row):
                if cell == "#":
                    self.active_states.add((x, y, 0))

    def is_active(self, x, y, z):
        return (x, y, z) in self.active_states

    def get_corner(self, z_slice):
        x_max = None
        x_min = None

        y_max = None
        y_min = None

        for x, y, z in self.active_states:
            if z == z_slice:
                if x_max is None or x > x_max:
                    x_max = x
                
                if x_min is None or x < x_min:
                    x_min = x
                
                if y_max is None or y > y_max:
                    y_max = y
                
                if y_min is None or y < y_min:
                    y_min = y

        return ((x_min, y_min), (x_max, y_max))
    
    def print_slice(self, z_slice):
        (x_min, y_min), (x_max, y_max) = self.get_corner(z_slice)

        for x in range(x_min, x_max + 1):
            for y in range(y_min, y_max + 1):
                print("#" if self.is_active(x, y, z_slice) else ".", end="")
            print()

    def get_active_slices(self):
        slices = set()
        for x, y, z in self.active_states:
            slices.add(z)

        return sorted(slices)


    def do_step(self):
        adjacencies = {}

        steps = []

        for i in range(-1, 2):
            for j in range(-1, 2):
                for k in range(-1, 2):
                    if i != 0 or j != 0 or k != 0:
                        steps.append((i, j, k))

        for x, y, z in self.active_states:
            if (x, y, z) not in adjacencies:
                adjacencies[(x, y, z)] = 0

            for dx, dy, dz in steps:
                cell = (x + dx, y + dy, z + dz)

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


    def print_all_slices(self):
        for z in self.get_active_slices():
            print(f"{z=}")
            self.print_slice(z)
            print()

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