from copy import deepcopy

INPUT_FILE = "./puzzle.txt"

def is_first_seen_occupied(seats, r, c, dr, dc):
    height = len(seats)
    width = len(seats[0])

    steps = 1

    while 0 <= r + steps * dr < height and 0 <= c + steps * dc < width:

        seat = seats[r + steps * dr][c + steps * dc]

        if seat == "#":
            return True
        elif seat == "L":
            return False

        steps += 1

    return False


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
                        if (dr != 0 or dc != 0) and is_first_seen_occupied(seats, r, c, dr, dc):
                            occupied_neighbours += 1

                if cell == "L" and occupied_neighbours == 0:
                    next_state[r][c] = "#"
                    changes += 1
                elif cell == "#" and occupied_neighbours >= 5:
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