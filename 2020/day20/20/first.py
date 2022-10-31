import numpy as np

from copy import deepcopy

INPUT_FILE = "./test.txt"


def load_tiles():
    tiles = {}
    with open(INPUT_FILE, "r") as infile:
        with open(INPUT_FILE, "r") as infile:
            for tile_data in infile.read().split("\n\n"):
                tile_id = int(tile_data.split(":")[0].split(" ")[1])
                tile_text = tile_data.split(":")[1].strip()

                tile = []

                for row in tile_text.splitlines():
                    r = []
                    for cell in row.strip():
                        r.append(0 if cell == "." else 1)
                    tile.append(r)

                tiles[tile_id] = np.array(tile)
    
    return tiles


def clone_without(dictionary, removed_key):
    new_dict= {k: dictionary[k] for k in dictionary if k != removed_key}
    return new_dict


def main():
    tiles = load_tiles()
    remaining_tiles = frozenset(tiles.keys())

    N = int(np.sqrt(len(tiles)))
    if N * N != len(tiles):
        raise Exception("Tile number should be perfect square")

    config = [ [(0,0) for _ in range(3)] for _ in range(3)]
    print(config)

    config[0][0] = (1951, 2)

    def get_tile(tdata):
        tile = tiles[tdata[0]]
        for _ in range(tdata[1]):
            tile = np.rot90(tile)

        return tile


    def is_side_compatible(t1, t2, side):
        pass


    def try_fit(config, tid, rotation, position):
        tile = get_tile((tid, rotation))
        r, c = position

        N = len(config)
        

        for dr, dc in [ (1, 0), (-1, 0), (0, 1), (0, -1) ]:
            nr, nc = r + dr, c + dc

            if 0 <= nr < N and 0 <= nc < N and config[nr][nc][0] != 0:
                if not is_side_compatible(tile, get_tile(config[nr][nc]), (dr, dc)):
                    return False

        return True


        



    def solve(config, remaining, next_pos):
        # for tid in remaining:

        print(try_fit(config, 2311, 0, next_pos))
            



        print(get_tile(config[0][0]))
        print(remaining)


    result = solve(config, remaining_tiles.difference({1951}), (0, 1))

    print(result)


if __name__ == "__main__":
    main()