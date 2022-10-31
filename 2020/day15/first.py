INPUT_FILE = "./puzzle.txt"

LIMIT = 2020

def find_last_occurence(item, array):
    for i in range(len(array) - 1, -1, -1):
        if array[i] == item:
            return i
    
    raise Exception("Can't find number")

def main():
    with open(INPUT_FILE, "r") as infile:
        numbers = list(map(lambda x: int(x), infile.readline().strip().split(",")))

    turn = len(numbers)
    last = numbers.pop()

    while turn < LIMIT:
        if last not in numbers:
            numbers.append(last)
            last = 0
        else:
            tmp = turn - (find_last_occurence(last, numbers) + 1)

            numbers.append(last)
            last = tmp
        
        turn += 1

    print(last)

if __name__ == "__main__":
    main()