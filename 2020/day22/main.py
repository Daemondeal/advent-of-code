from copy import copy

def main():
    print(part_1())
    print(part_2())

def part_2():
    player1, player2 = load_file("input.txt")

    return play(player1, player2)[1]

def play(player1, player2, depth=0):
    past_states = set()

    while len(player1) > 0 and len(player2) > 0:
        if (tuple(player1), tuple(player2)) in past_states:
            return True, 0
        past_states.add((tuple(player1), tuple(player2)))

        c1 = player1.pop(0)
        c2 = player2.pop(0)

        if len(player1) >= c1 and len(player2) >= c2:
            player1_won, _ = play(player1.copy()[:c1], player2.copy()[:c2], depth + 1)
        else:
            player1_won = c1 > c2

        if player1_won:
            player1.append(c1)
            player1.append(c2)
        else:
            player2.append(c2)
            player2.append(c1)

    score = 0
    if depth == 0:
        winner = player1 if len(player1) > 0 else player2
        score = sum([x * (len(winner)-i) for i, x in enumerate(winner)])

    return len(player1) > 0, score




def part_1():
    player1, player2 = load_file("input.txt")

    round = 1
    while len(player1) > 0 and len(player2) > 0:
        c1 = player1.pop(0)
        c2 = player2.pop(0)

        if c1 > c2:
            player1.append(c1)
            player1.append(c2)
        else:
            player2.append(c2)
            player2.append(c1)

        round += 1

    winner = player1 if len(player1) > 0 else player2

    score = 0
    for i, x in enumerate(winner):
        score += x * (len(winner)-i)

    return score

def load_file(file_name):
    player1 = []
    player2 = []

    with open(file_name, "r") as infile:
        players = infile.read().split("Player 2:")

        for line in players[0].split("\n")[1:]:
            if line.strip() != "":
                player1.append(int(line.strip()))

        for line in players[1].split("\n"):
            if line.strip() != "":
                player2.append(int(line.strip()))

    return player1, player2


if __name__ == "__main__":
    main()