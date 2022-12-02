import os
import strutils
import sequtils

proc points(turn: string): int =
    case turn:
        of "AX":
            return 3 + 1
        of "AY":
            return 6 + 2
        of "AZ":
            return 0 + 3
        of "BX":
            return 0 + 1
        of "BY":
            return 3 + 2
        of "BZ":
            return 6 + 3
        of "CX":
            return 6 + 1
        of "CY":
            return 0 + 2
        of "CZ":
            return 3 + 3

    return 0

proc pointsPartTwo(turn: string): int =
    case turn:
        of "AX":
            return 0 + 3
        of "AY":
            return 3 + 1
        of "AZ":
            return 6 + 2
        of "BX":
            return 0 + 1
        of "BY":
            return 3 + 2
        of "BZ":
            return 6 + 3
        of "CX":
            return 0 + 2
        of "CY":
            return 3 + 3
        of "CZ":
            return 6 + 1

    return 0

proc processInput(input: string): seq[string] =
    return input.split("\n").map(proc (x: string): string = x.strip().replace(" ", ""))

proc solveA(input: string): int =
    return foldl(processInput(input).map(points), a + b)

proc solveB(input: string): int =
    return foldl(processInput(input).map(pointsPartTwo), a + b)

proc main() =
    if paramCount() != 1:
        echo "USAGE: nim r main.nim [input_file]"
    
    let fileContent = readFile(paramStr(1))

    echo "A: ", solveA(fileContent)
    echo "B: ", solveB(fileContent)

main()