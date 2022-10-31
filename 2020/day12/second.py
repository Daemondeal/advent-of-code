from math import sin, cos, radians

INPUT_FILE = "./puzzle.txt"

def rotate_vector(vec, deg):
    x, y = vec
    s, c = int(sin(radians(deg))), int(cos(radians(deg)))

    return (x * c - y * s, x * s + y * c)

def main():
    pos_north = 0
    pos_east = 0

    waypoint_north = 1
    waypoint_east = 10


    with open(INPUT_FILE, "r") as infile:
        for line in infile:
            op = line[0]
            amount = int(line[1:].strip())

            if op == "N":
                waypoint_north += amount
            elif op == "S":
                waypoint_north -= amount
            elif op == "E":
                waypoint_east += amount
            elif op == "W":
                waypoint_east -= amount

            elif op == "R":
                waypoint_east, waypoint_north = rotate_vector(
                    (waypoint_east, waypoint_north),
                    360 - amount
                )

            elif op == "L":
                waypoint_east, waypoint_north = rotate_vector(
                    (waypoint_east, waypoint_north),
                    amount
                )

            elif op == "F":
                x, y = waypoint_east, waypoint_north
                pos_east += x * amount
                pos_north += y * amount
        
                
    print(pos_north, pos_east)
    print(abs(pos_east) + abs(pos_north))


if __name__ == "__main__":
    main()