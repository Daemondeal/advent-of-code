import numpy as np
from math import prod, sqrt
from copy import copy

def main():
    print("A:", solve_1())
    print("B:", solve_2())

def compose_bitmap(total_map, tiles):
    sample = next(iter(tiles.values()))

    side = sample.shape[0] - 2

    tilemap = np.zeros((side * total_map.shape[0], side * total_map.shape[1]), dtype=int)

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
    ), dtype=int)

def solve_2():
    tiles = read_file('input.txt')

    total_map = compose_map(tiles)

    bitmap = map_to_bitmap(total_map)

    monsters = count_monsters(bitmap)

    return np.sum(bitmap) - np.sum(KERNEL) * monsters

def count_monsters(bitmap):
    max_monsters = 0

    kw, kh = KERNEL.shape
    for rot in rotations(bitmap):
        monsters = 0
        width, height = rot.shape
        
        for i in range(width - kw + 1):
            for j in range(height - kh + 1):
                valid = True

                for ki in range(kw):
                    for kj in range(kh):
                        if KERNEL[ki, kj] == 1 and rot[i + ki, j + kj] != 1:
                            valid = False

                if valid:
                    monsters += 1

        if monsters > max_monsters:
            max_monsters = monsters
        
    return monsters


def map_to_bitmap(total_map):
    inner_tile = total_map[0][0][1].shape[0] - 2
    side = (inner_tile) * len(total_map)

    bitmap = np.zeros((side, side), dtype=int)

    for i, row in enumerate(total_map):
        for j, cell in enumerate(row):

            tile = cell[1]
            for ti in range(inner_tile):
                for tj in range(inner_tile):
                    bitmap[i * inner_tile + ti, j * inner_tile + tj] = tile[ti + 1, tj + 1]

    return bitmap



def compose_map(tiles):
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
    
    # Solve top
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


    for _ in range(1, 12):
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

    final_map = [top_row]
    for _ in range(1, 12):
        final_map.append([(9999, [])] * len(top_row))
    
    
    # Solve columns from top
    for t in range(0, 12):
        cur_id = top_row[t][0]
        current = top_row[t][1]

        col = []

        for _ in range(1, 12):
            bot = current[-1, :]
            matching = [x for x in edges if x != cur_id and edges[x] & {tuple(bot), reversed(tuple(bot))}][0]

            for rot in rotations(tiles[matching]):
                top = rot[0, :]
                if np.array_equal(top, bot):
                    new_tile = rot
                    break
            col.append((matching, new_tile))
            cur_id = matching
            current = new_tile

        for i, x in enumerate(col):
            final_map[i + 1][t] = x

    return final_map
    

def rotations(tile):
    for _ in range(2):
        for _ in range(4):
            yield tile
            tile = np.rot90(tile)
        tile = np.flip(tile, 0)

def read_file(filename):
    tiles = {}

    with open(filename, 'r') as infile:
        for tile in infile.read().split("Tile"):
            if len(tile.strip()) == 0:
                continue

            id, body = tile.split(':')
            
            lines = body.strip().split("\n")

            tile_arr = np.zeros((len(lines), len(lines[0])), dtype=int)

            for i in range(len(lines)):
                for j in range(len(lines[i])):
                    tile_arr[i][j] = lines[i][j] == '#'

            tiles[int(id.strip())] = tile_arr

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