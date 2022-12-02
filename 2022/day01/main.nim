import os
import std/strutils
import algorithm
import sequtils

proc processInput(input: string): seq[seq[int]] =
    var output: seq[seq[int]] = @[]

    for line in input.split("\n\n"):
        output.add(line.strip().split("\n").map(parseInt))

    return output

proc solveA(input: string): int =
    let elfs =  processInput(input)


    return elfs.map(proc (xs: seq[int]): int = foldl(xs, a + b)).max()

proc solveB(input: string): int =
    let elfs =  processInput(input)    
    var calories = elfs.map(proc (xs: seq[int]): int = foldl(xs, a + b))
        
    sort(calories, system.cmp[int], Descending)


    return calories[0] + calories[1] + calories[2]

proc main() =
    if paramCount() != 1:
        echo "USAGE: nim r main.nim [input_file]"
    
    let fileContent = readFile(paramStr(1))

    echo "A: ", solveA(fileContent)
    echo "B: ", solveB(fileContent)

main()