INPUT_FILE = "./puzzle.txt"

LIMIT = 30000000

def find_last_occurence(item, array):
    for i in range(len(array) - 1, -1, -1):
        if array[i] == item:
            return i
    
    raise Exception("Can't find number")

def main():
    with open(INPUT_FILE, "r") as infile:
        numbers = list(map(lambda x: int(x), infile.readline().strip().split(",")))

    last_occurences = {}
    for i, n in enumerate(numbers):
        last_occurences[n] = i + 1

    turn = len(numbers)
    last = numbers.pop()

    while turn < LIMIT:
        if LIMIT > 10000 and turn % (LIMIT // 100) == 0:
            print(f"Progress: {round((float(turn) / LIMIT) * 100, 2)}%")

        if last in last_occurences:
            tmp = turn - last_occurences[last]
            last_occurences[last] = turn
            last = tmp
        else:
            last_occurences[last] = turn
            
            last = 0

        turn += 1


    print(last)


    return

    while turn < LIMIT:
        if LIMIT > 10000 and turn % (LIMIT // 1000) == 0:
            print(f"Progress: {round((float(turn) / LIMIT) * 100, 2)}%")


        if last not in spoken_numbers:
            numbers.append(last)
            spoken_numbers.add(last)
            last = 0
        else:
            tmp = turn - (find_last_occurence(last, numbers) + 1)

            numbers.append(last)
            spoken_numbers.add(last)
            last = tmp
        
        turn += 1

    print(numbers[:10], last)

if __name__ == "__main__":
    main()