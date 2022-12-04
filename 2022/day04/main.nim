import os
import strutils

type ElfRange = tuple
    rangeStart: int
    rangeEnd: int

func fullyContains(a: ElfRange, b: ElfRange): bool =
    return a.rangeStart <= b.rangeStart and a.rangeEnd >= b.rangeEnd

func overlap(a: ElfRange, b: ElfRange): int =
    if a.rangeStart > b.rangeStart:
        return max(b.rangeEnd - a.rangeStart + 1, 0)
    else:
        return max(a.rangeEnd - b.rangeStart + 1, 0)

proc processInput(input: string): seq[tuple[left: ElfRange, right: ElfRange]] =
    var elves: seq[tuple[left: ElfRange, right: ElfRange]] = @[]
    
    for line in input.strip().split("\n"):
        let
            lineElves = line.strip().split(",")
            leftRange = lineElves[0].split("-")
            rightRange = lineElves[1].split("-")

        elves.add((
            left: (rangeStart: leftRange[0].parseInt, rangeEnd: leftRange[1].parseInt),
            right: (rangeStart: rightRange[0].parseInt, rangeEnd: rightRange[1].parseInt)
        ))

    return elves


proc solveA(input: string): int =
    let elves = processInput(input)

    var count = 0

    for elf in elves:
        if elf.left.fullyContains(elf.right) or elf.right.fullyContains(elf.left):
            count += 1

    return count

proc solveB(input: string): int =
    let elves = processInput(input)

    var count = 0

    for elf in elves:
        if elf.left.overlap(elf.right) > 0:
            count += 1 

    return count

proc main() =
    if paramCount() != 1:
        echo "USAGE: nim r main.nim [input_file]"
    
    let fileContent = readFile(paramStr(1))

    echo "A: ", solveA(fileContent)
    echo "B: ", solveB(fileContent)

main()