INPUT_FILE = "./puzzle.txt"

def main():

    with open(INPUT_FILE, "r") as infile:
        adapters = set(int(x.strip()) for x in infile)
    
    final_adapter = max(adapters) + 3
    current_adapter = 0

    delta_ones = 0
    delta_threes = 0

    while len(adapters) > 0:
        if current_adapter + 1 in adapters:
            current_adapter = current_adapter + 1
            delta_ones += 1

        elif current_adapter + 2 in adapters:
            current_adapter = current_adapter + 2

        elif current_adapter + 3 in adapters:
            current_adapter = current_adapter + 3
            delta_threes += 1
        else:
            print("Impossible")
            exit()


        adapters.remove(current_adapter)
    
    delta_threes += 1

    print(delta_ones, delta_threes, delta_ones * delta_threes)


if __name__ == "__main__":
    main()