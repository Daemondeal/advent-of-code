INFILE_NAME = "./puzzle.txt"

def main():
    nums = []

    with open(INFILE_NAME, "r", encoding='utf8') as infile:
        for line in infile:
            if line != "":
                nums.append(int(line))

    nums_set = set(nums)

    found = False

    for n in nums:
        for m in nums:
            if (2020 - n - m) in nums_set:
                print(n * m * (2020 - n - m))
                found = True
                break
        if found:
            break



if __name__ == "__main__":
    main()