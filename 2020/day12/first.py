from math import sin, cos, radians

INPUT_FILE = "./puzzle.txt"

def rotate_vector(vec, deg):
    x, y = vec
    s, c = int(sin(radians(deg))), int(cos(radians(deg)))

    return (x * c - y * s, x * s + y * c)

def main():
    pos_north = 0
    pos_east = 0

    direction = (1, 0)

    with open(INPUT_FILE, "r") as infile:
        for line in infile:
            op = line[0]
            amount = int(line[1:].strip())

            if op == "N":
                pos_north += amount
            elif op == "S":
                pos_north -= amount
            elif op == "E":
                pos_east += amount
            elif op == "W":
                pos_east -= amount

            elif op == "R":
                direction = rotate_vector(direction, 360 - amount)
            elif op == "L":
                direction = rotate_vector(direction, amount)
            elif op == "F":
                x, y = direction
                pos_east += x * amount
                pos_north += y * amount
            elif op == "B":
                x, y = direction
                pos_east -= x * amount
                pos_north -= y * amount
            

                
    print(pos_north, pos_east)
    print(abs(pos_east) + abs(pos_north))


if __name__ == "__main__":
    main()