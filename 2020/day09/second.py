INPUT_FILE = "./puzzle.txt"
PREAMBLE_LENGTH = 25

def is_sum(preamble, num):
    for n in preamble:
        if (num - n) in preamble and (num - n) != n:
            return True
    
    return False

def main():
    with open(INPUT_FILE, "r") as infile:
        preamble = []
        numbers = list(map(lambda x: int(x.strip()), infile.readlines()))
        invalid_number = -1

        for num in numbers[:PREAMBLE_LENGTH]:
            preamble.append(num)

        for num in numbers[PREAMBLE_LENGTH:]:
            if not is_sum(preamble, num):
                invalid_number = num
                break
            
            preamble.pop(0)
            preamble.append(num)
    
    found = False

    for i, num in enumerate(numbers):
        if num == invalid_number:
            continue

        j = 0
        running_sum = 0

        while running_sum < invalid_number and i + j < len(numbers):
            running_sum += numbers[i + j]
            if running_sum == invalid_number:
                found = True
                weak_list = numbers[i:i + j + 1]
                print(max(weak_list) + min(weak_list))
                break
            j += 1


        if found:
            break



if __name__ == "__main__":
    main()