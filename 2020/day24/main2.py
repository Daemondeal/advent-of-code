from enum import Enum, auto

class Direction(Enum):
    E  = auto()
    SE = auto()
    SW = auto()
    W  = auto()
    NW = auto()
    NE = auto()


def dir_to_vec(dir):
    if dir == Direction.E:
        return (1, 0)
    elif dir == Direction.SE:
        return (0, 1)
    elif dir == Direction.SW:
        return (-1, 1)
    elif dir == Direction.W:
        return (-1, 0)
    elif dir == Direction.NW:
        return (0, -1)
    elif dir == Direction.NE:
        return (1, -1)


MAP_SIZE = 300

def prepare_map(filename):
    map = []
    for i in range(MAP_SIZE):
        map.append([False] * MAP_SIZE)

    tiles = read_file(filename)
    for moves in tiles:
        q = MAP_SIZE // 2
        r = MAP_SIZE // 2

        for move in moves:
            (dq, dr) = dir_to_vec(move)
            q += dq
            r += dr
        map[q][r] = not map[q][r]
    
    return map

def get_active(map):
    return sum(sum(cell for cell in row) for row in map)

def solve_1():
    return get_active(prepare_map('input.txt'))

ITERS = 100

def solve_2():
    map = prepare_map('input.txt')

    neighbours = [
        (1, 0),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (0, -1),
        (1, -1)
    ]


    for _ in range(ITERS):  
        print(_)  
        to_flip = []
        for q in range(MAP_SIZE):
            for r in range(MAP_SIZE):
                cell = map[q][r]
                black = 0

                for dq, dr in neighbours:
                    qn = q + dq
                    rn = r + dr

                    if 0 <= qn < MAP_SIZE and 0 <= rn < MAP_SIZE and map[qn][rn]:
                        black += 1

                if cell:
                    if black == 0 or black > 2:
                        to_flip.append((q, r))
                else:
                    if black == 2:
                        to_flip.append((q, r))

        for q, r in to_flip:
            map[q][r] = not map[q][r]

    return get_active(map)

def main():
    print(solve_2())

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