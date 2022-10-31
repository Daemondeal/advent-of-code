def main():
    print(part_1(157623984))

def part_1(labeling):
    cups = [int(x) for x in str(labeling)]

    for _ in range(100):
        taken = cups[1:4]
        cups = [cups[0]] + cups[4:]
        
        dest = cups[0] - 1
        while dest not in cups:
            dest -= 1
            dest += max(cups) + 1
            dest %= max(cups) + 1
        
        # print(_, dest)

        dest_index = cups.index(dest) + 1

        cups = cups[1:dest_index] + taken + cups[dest_index:] + [cups[0]]
    
    one_index = cups.index(1)
    res = cups[one_index:] + cups[:one_index]
    return "".join([str(x) for x in res[1:]])

if __name__ == "__main__":
    main()