from enum import Enum, auto

class ConvenientMap:
    def __init__(self, init_param=0):
        self.init_param = init_param
        self._map = {}

    def set(self, key, value):
        self._map[key] = value
    
    def get(self, key):
        if key not in self._map:
            self._map[key] = self.init_param

        return self._map[key]

    def get_imm(self, key):
        if key not in self._map:
            return self.init_param
        return self._map

    def update(self, key, val):
        self.set(key, self.get(key) + val)

    def get_inner(self):
        return self._map

    def values(self):
        return self._map.values()
    
    def __str__(self) -> str:
        return f"Init({self.init_param}, {str(self._map)})"

class Direction(Enum):
    E  = auto()
    SE = auto()
    SW = auto()
    W  = auto()
    NW = auto()
    NE = auto()

def main():
    print(solve_1())
    # print(solve_2())

def solve_1():
    return sum(setup_map('input.txt').values()) 

def solve_2():
    map = setup_map('test.txt')

    neighbours = [
        (1, -1, 0),
        (1, 0, -1),
        (0, 1, -1),
        (-1, 1, 0),
        (-1, 0, 1),
        (0, -1, 1)
    ]

    for _ in range(1):
        adj_map = ConvenientMap(0)
        to_flip = []
        for pos in map.get_inner():
            if map.get(pos) == True:
                x, y, z = pos

                black_count = 0

                for dx, dy, dz in neighbours:
                    neighbour_pos = (x + dx, y + dy, z + dz)
                    
                    if map.get_imm(neighbour_pos) == False:
                        adj_map.update(neighbour_pos, 1)
                    else:
                        black_count += 1
                
                if black_count == 0 or black_count > 2:
                    to_flip.append(pos)
        
        print(len(to_flip))
        for pos in adj_map.get_inner():
            if adj_map.get(pos) == 2:
                print(map.get(pos))
                to_flip.append(pos)

        for pos in to_flip:
            map.set(pos, not map.get(pos))




    print(sum(map.values()))

def setup_map(filename):
    tiles = read_file(filename)
    
    map = ConvenientMap(False)

    for moves in tiles:
        x = 0
        y = 0
        z = 0

        for move in moves:
            if move == Direction.E:
                x += 1
                y -= 1
            elif move == Direction.SE:
                y -= 1
                z += 1
            elif move == Direction.SW:
                x -= 1
                z += 1
            elif move == Direction.W:
                x -= 1
                y += 1
            elif move == Direction.NW:
                y += 1
                z -= 1
            elif move == Direction.NE:
                x += 1
                z -= 1

        map.set((x, y, z), not map.get((x, y, z)))

    return map

def read_file(filename):
    with open(filename, 'r') as infile:
        tot_moves = []
        for line in infile:
            text = line.strip()
            moves = []
            i = 0
            while i < len(text):
                if text[i] == 'e':
                    moves.append(Direction.E)
                    i += 1
                elif text[i] == 'w':
                    moves.append(Direction.W)
                    i += 1
                elif text[i] == 's':
                    if text[i + 1] == 'e':
                        moves.append(Direction.SE)
                        i += 2
                    elif text[i + 1] == 'w':
                        moves.append(Direction.SW)
                        i += 2
                elif text[i] == 'n':
                    if text[i + 1] == 'e':
                        moves.append(Direction.NE)
                        i += 2
                    elif text[i + 1] == 'w':
                        moves.append(Direction.NW)
                        i += 2


            tot_moves.append(moves)
    return tot_moves


if __name__ == "__main__":
    main()