import os
import sequtils
import strutils
import std/sets

proc toPriority(input: char): int =
    let ascii = ord(input)
    if ascii < 91:
        return ascii - 64 + 26
    else:
        return ascii - 96

proc processInput(input: string): seq[tuple[left: HashSet[int], right: HashSet[int]]] =
    var rucksacks: seq[tuple[left: HashSet[int], right: HashSet[int]]] = @[]

    for line in input.strip().split("\n"):
        let 
            line = line.strip()
            itemCount = line.len
            halfPoint = int(itemCount/2)

        rucksacks.add((
            left: toHashSet(line[0 ..< halfPoint ].map(toPriority)),
            right: toHashSet(line[halfPoint .. itemCount - 1].map(toPriority))
        ))

    return rucksacks

proc solveA(input: string): int =
    let rucksacks = processInput(input)

    var prioritySum = 0

    for (left, right) in rucksacks:
        prioritySum += (left * right).toSeq().foldl(a + b)


    return prioritySum

proc solveB(input: string): int =
    let rucksacks = processInput(input)
    var prioritySum = 0

    for i in 0 ..< int(rucksacks.len / 3):
        let 
            r1 = rucksacks[i * 3]
            r2 = rucksacks[i * 3 + 1]
            r3 = rucksacks[i * 3 + 2]

            r1h = r1.left + r1.right
            r2h = r2.left + r2.right
            r3h = r3.left + r3.right

        prioritySum += (r1h * r2h * r3h).toSeq().foldl(a + b)
        
    return prioritySum

proc main() =
    if paramCount() != 1:
        echo "USAGE: nim r main.nim [input_file]"
    
    let fileContent = readFile(paramStr(1))

    echo "A: ", solveA(fileContent)
    echo "B: ", solveB(fileContent)

main()