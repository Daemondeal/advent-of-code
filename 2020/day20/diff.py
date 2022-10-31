from termcolor import colored

def open_mat(filename):
    with open(filename, 'r') as infile:
        mat = []
        for line in infile:
            row = [int(x) for x in line.strip().split(" ")]
            mat.append(row)

    return mat

other = open_mat('other.txt')
mine = open_mat('mine.txt')

for i, row in enumerate(other):
    for j, col in enumerate(row):
        color = 'white'
        if other[i][j] != mine[i][j]:
            color = 'red'

        print(colored(mine[i][j], color), end=" ")
    print()
        