INFILE_NAME = "./puzzle.txt"

def main():
    nums = []

    with open(INFILE_NAME, "r", encoding='utf8') as infile:
        for line in infile:
            if line != "":
                nums.append(int(line))

    nums_set = set(nums)

    for n in nums:
        if (2020 - n) in nums_set:
            print(n * (2020 - n))
            break



if __name__ == "__main__":
    main()