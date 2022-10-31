from enum import Enum

class Corner(Enum):
    RIGHT = 0
    LEFT = 1
    TOP = 2
    BOTTOM = 3

    @staticmethod
    def get_opposite(corner):
        if corner == Corner.RIGHT:
            return Corner.LEFT
        elif corner == Corner.LEFT:
            return Corner.RIGHT
        elif corner == Corner.TOP:
            return Corner.BOTTOM
        else:
            return Corner.TOP
    

INPUT_FILE = "./test.txt"

class Tile:
    def __init__(self, tile_id: int, tile_data: str):
        self.id = tile_id
        self.tile_data = []

        for line in tile_data.splitlines():
            self.tile_data.append(list(line.strip()))

    def get_id(self):
        return self.id

    def get_width(self):
        return len(self.tile_data[0])
    
    def get_height(self):
        return len(self.tile_data)

    def print_tile(self):
        for row in self.tile_data:
            for cell in row:
                print(cell, end="")
            print()

    def rotate_90deg(self):
        N = self.get_width()

        for i in range(N // 2):
            for j in range (i, N - i - 1):
                temp = self.tile_data[i][j]
                self.tile_data[i][j] = self.tile_data[N - 1 - j][i]
                self.tile_data[N - 1 - j][i] = self.tile_data[N - 1 - i][N - 1 - j]
                self.tile_data[N - 1 - i][N - 1 - j] = self.tile_data[j][N - 1 - i]
                self.tile_data[j][N - 1 - i] = temp

    def get_corner(self, corner):
        corner_data = []
        N = self.get_width()

        if corner == Corner.TOP:
            for i in range(0, N):
                corner_data.append(self.tile_data[0][i])
        elif corner == Corner.BOTTOM:
            for i in range(0, N):
                corner_data.append(self.tile_data[N-1][i])
        elif corner == Corner.LEFT:
            for i in range(0, N):
                corner_data.append(self.tile_data[i][0])
        elif corner == Corner.RIGHT:
            for i in range(0, N):
                corner_data.append(self.tile_data[0][N-1])

        return "".join(corner_data)

def main():
    tiles = []

    with open(INPUT_FILE, "r") as infile:
        for tile_data in infile.read().split("\n\n"):
            tile_id = int(tile_data.split(":")[0].split(" ")[1])
            tile_data = tile_data.split(":")[1].strip()

            tiles.append(Tile(tile_id, tile_data))


    # tiles[0].print_tile()

    cur = tiles[0]
    corners = [
        Corner.LEFT,
        Corner.RIGHT,
        Corner.TOP,
        Corner.BOTTOM
    ]

    tile_num = {}

    a = next(iter(filter(lambda x: x.get_id() == 1951, tiles)))
    b = next(iter(filter(lambda x: x.get_id() == 2311, tiles)))

    a.rotate_90deg()
    a.rotate_90deg()
    a.print_tile()

    print()
    b.print_tile()

    return

    for cur in tiles:
        tile_num[cur.get_id()] = 0
        for tile in tiles:
            if tile != cur:
                for c1 in corners:
                    for c2 in corners:
                        if cur.get_corner(c1) == tile.get_corner(c2):
                            tile_num[cur.get_id()] += 1
                            print(cur.get_id(), tile.get_id(), c1, c2)

    print(tile_num)
    
    # print()
    # print(tiles[0].get_corner(Corner.LEFT))
    # print()
    # print(tiles[0].get_corner(Corner.RIGHT))
    # print()
    # print(tiles[0].get_corner(Corner.TOP))
    # print()
    # print(tiles[0].get_corner(Corner.BOTTOM))

if __name__ == "__main__":
    main()