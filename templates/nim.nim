import os

proc processInput(input: string) =
    discard input

proc solveA(input: string): int =
    return 0

proc solveB(input: string): int =
    return 0

proc main() =
    if paramCount() != 1:
        echo "USAGE: nim r main.nim [input_file]"
    
    let fileContent = readFile(paramStr(1))

    echo "A: ", solveA(fileContent)
    echo "B: ", solveB(fileContent)

main()