import numpy as np
from math import prod, sqrt
from copy import copy

def main():
    # print(solve_1())
    # print(solve_2())
    solve_2()

def compose_bitmap(total_map, tiles):
    sample = next(iter(tiles.values()))

    side = sample.shape[0] - 2

    tilemap = np.zeros((side * total_map.shape[0], side * total_map.shape[1]))

    for i, row in enumerate(total_map):
        for j, cell_f in enumerate(row):
            cell = int(cell_f)

            for ti in range(side):
                for tj in range(side):
                    tt = tiles[str(cell)]
                    # print(tt)

                    tilemap[i * side + ti, j * side + tj] = tt[ti + 1, tj + 1]

    return tilemap

KERNEL = np.array((
        [1 if x == "#" else 0 for x in "                  # "],
        [1 if x == "#" else 0 for x in "#    ##    ##    ###"],
        [1 if x == "#" else 0 for x in " #  #  #  #  #  #   "]
    ))

def solve_2():
    tiles = read_file('input.txt')

    test = solve_top(tiles)

    # total_map = compose_map(copy(tiles))

    # for row in total_map:
    #     for cell in row:
    #         print(int(cell), end=" ")
    #     print()


def solve_top(tiles):
    edges = {}
    for tid, tile in tiles.items():
        tile_edges = set()

        for edge in [tile[0], tile[-1], tile[:,0], tile[:,-1]]:
            tile_edges.add(tuple(edge))
            tile_edges.add(tuple(reversed(edge)))
        
        edges[tid] = tile_edges

    neighbours = {}
    for t1 in tiles:
        neighs = set()
        for t2 in tiles:
            if t1 == t2:
                continue

            if edges[t1].intersection(edges[t2]):
                neighs.add(t2)
        neighbours[t1] = neighs

    corners = [x for x in neighbours if len(neighbours[x]) == 2]

    length = int(sqrt(len(tiles)))
    
    
    final_map = []

    cur_id = corners[0]
    current = tiles[cur_id]


    for rot in rotations(current):
        bot = rot[-1, :]
        right = rot[:, -1]
        
        cur_edges = {
            tuple(bot), tuple(right), reversed(tuple(bot)), reversed(tuple(right))
        }

        cur_neighs = [
            x 
            for x in edges
            if x != cur_id and cur_edges & edges[x]
        ]

        if len(cur_neighs) == 2:
            current = rot
            break

    top_row = []
    top_row.append((cur_id, copy(current)))


    for x in range(1, 12):
        right = current[:,-1]

        matching = [x for x in edges if x != cur_id and edges[x] & {tuple(right), reversed(tuple(right))}][0]

        for rot in rotations(tiles[matching]):
            left = rot[:, 0]
            if np.array_equal(right, left):
                new_tile = rot
                break
                
        top_row.append((matching, new_tile))
        cur_id = matching
        current = new_tile
        

    
    print([x[0] for x in top_row])

def rotations(tile):
    for _ in range(2):
        for _ in range(4):
            yield tile
            tile = np.rot90(tile)
        tile = np.flip(tile, 0)



def compose_map(tiles):
    length = int(sqrt(len(tiles)))
    total_map = np.zeros((length, length))

    tiles2 = copy(tiles)

    k = 0
    while len(tiles) > 0:
        frame, visited = solve_frame(tiles)
        if k > 0:

            t1 = tiles2[str(int(total_map[k + 1,k-1]))]

            t1edges = get_edges(t1)
            
            found = False
            for _ in range(2):
                for _ in range(4):
                    b2 = str(int(frame[1,0]))
                    t2 = tiles2[b2]
                    t2edges = get_edges(t2)

                    if t1edges.intersection(t2edges):
                        found = True
                        break

                    frame = np.rot90(frame)

                if found:
                    break
                frame = np.flip(frame, 0)



        for i, line in enumerate(frame):
            for j, cell in enumerate(line):
                if cell != 0:
                    total_map[i + k, j + k] = cell

        for tile in visited:
            tiles.pop(tile)
        
        k += 1

    return total_map

def solve_frame(tiles):
    edges = {}
    for tid, tile in tiles.items():
        tile_edges = set()

        for edge in [tile[0], tile[-1], tile[:,0], tile[:,-1]]:
            tile_edges.add(tuple(edge))
            tile_edges.add(tuple(reversed(edge)))
        
        edges[tid] = tile_edges

    neighbours = {}
    for t1 in tiles:
        neighs = set()
        for t2 in tiles:
            if t1 == t2:
                continue

            if edges[t1].intersection(edges[t2]):
                neighs.add(t2)
        neighbours[t1] = neighs

    corners = [x for x in neighbours if len(neighbours[x]) == 2]

    length = int(sqrt(len(tiles)))
    
    final_map = np.zeros((length, length))
    

    # Top left
    cur = corners[0]
    visited = {cur}
    final_map[0,0] = cur

    def has_corner(tid):
        return any(x for x in neighbours[tid] - visited if len(neighbours[x]) == 2)

    def get_side(tid):
        return [x for x in neighbours[tid] - visited if len(neighbours[x]) == 3][0]

    def get_corner(tid):
        return [x for x in neighbours[tid] - visited if len(neighbours[x]) == 2][0]

    # Left
    i = 1
    while not has_corner(cur):
        cur = get_side(cur)

        visited.add(cur)
        final_map[i, 0] = cur
        i += 1

    # Bottom left
    cur = get_corner(cur)
    final_map[-1, 0] = cur
    visited.add(cur)

    # Bottom
    i = 1
    while not has_corner(cur):
        cur = get_side(cur)

        visited.add(cur)
        final_map[-1, i] = cur
        i += 1

    # Bottom Right
    cur = get_corner(cur)
    final_map[-1, -1] = cur
    visited.add(cur)

    # Right
    i = 1
    while not has_corner(cur):
        cur = get_side(cur)

        visited.add(cur)
        final_map[-i -1, -1] = cur
        i += 1
        
    # Top Right
    cur = get_corner(cur)
    final_map[0, -1] = cur
    visited.add(cur)

    # Top
    i = 1
    while any(x for x in neighbours[cur] - visited if len(neighbours[x]) == 3):
        cur = get_side(cur)

        visited.add(cur)
        final_map[0, -i-1] = cur
        i += 1

    return final_map, visited

def get_edges(tile):
    tile_edges = set()

    for edge in [tile[0], tile[-1], tile[:,0], tile[:,-1]]:
        tile_edges.add(tuple(edge))
        tile_edges.add(tuple(reversed(edge)))

    return tile_edges

def read_file(filename):
    tiles = {}

    with open(filename, 'r') as infile:
        for tile in infile.read().split("Tile"):
            if len(tile.strip()) == 0:
                continue

            id, body = tile.split(':')
            
            lines = body.strip().split("\n")

            tile_arr = np.zeros((len(lines), len(lines[0])))

            for i in range(len(lines)):
                for j in range(len(lines[i])):
                    tile_arr[i][j] = lines[i][j] == '#'

            tiles[id.strip()] = tile_arr

    return tiles

def solve_1():
    tiles = read_file('input.txt')

    edges = {}

    for tid, tile in tiles.items():
        tile_edges = set()

        for edge in [tile[0], tile[-1], tile[:,0], tile[:,-1]]:
            tile_edges.add(tuple(edge))
            tile_edges.add(tuple(reversed(edge)))
        
        edges[tid] = tile_edges

    possible_neighbours = {}

    for t1 in tiles:
        neighs = set()
        for t2 in tiles:
            if t1 == t2:
                continue

            t1edg = edges[t1]
            t2edg = edges[t2]


            if t1edg.intersection(t2edg):
                neighs.add(t2)
        possible_neighbours[t1] = neighs

    corners = [x for x in possible_neighbours if len(possible_neighbours[x]) == 2]

    print(corners)
    return prod(int(x) for x in corners)

if __name__ == "__main__":
    main()