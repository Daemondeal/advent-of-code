INPUT_FILE = "./puzzle.txt"

def main():

    with open(INPUT_FILE, "r") as infile:
        adapters = sorted([0] + list(int(x.strip()) for x in infile))

    dp = [0] * len(adapters)

    dp[len(adapters) - 1] = 1

    for i in range(len(adapters) - 2, -1, -1):
        cur = adapters[i]

        for j in range(1, 4):
            if cur + j in adapters:
                dp[i] += dp[adapters.index(cur + j)]


    print(dp[0])
    


if __name__ == "__main__":
    main()