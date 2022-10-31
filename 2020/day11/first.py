from copy import deepcopy

INPUT_FILE = "./puzzle.txt"

def in_bounds(width, height, r, c):
    return 0 <= r < height and 0 <= c < width

def main():
    seats = []

    with open(INPUT_FILE, "r") as infile:
        for line in infile:
            seats.append(list(line.strip()))

    height = len(seats)
    width = len(seats[0])

    changes = -1

    while changes != 0:
        changes = 0
        next_state = deepcopy(seats)

        for r, row in enumerate(seats):
            for c, cell in enumerate(row):
                if cell == ".":
                    continue

                occupied_neighbours = 0
                
                for dr in [-1, 0, 1]:
                    for dc in [-1, 0, 1]:
                        if (dr != 0 or dc != 0) and (0 <= r + dr < height and 0 <= c + dc < width) and seats[r + dr][c + dc] == "#":
                            occupied_neighbours += 1

                if cell == "L" and occupied_neighbours == 0:
                    next_state[r][c] = "#"
                    changes += 1
                elif cell == "#" and occupied_neighbours >= 4:
                    next_state[r][c] = "L"
                    changes += 1
        
        seats = next_state
    

    occupied = 0
    for row in seats:
        for cell in row:
            if cell == "#":
                occupied += 1

    print(occupied)





if __name__ == "__main__":
    main()