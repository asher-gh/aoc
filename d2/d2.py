# rock paper scissor
p1 = {"A": 1, "B": 2, "C": 3}
p2 = {"X": 1, "Y": 2, "Z": 3}

# print(p1.get("B"))
final = 0

with open("./inputs/d2saved", "r") as f:
    lines = [line.strip() for line in f]

    for game in lines:
        pay = game.split(" ")
        a = p1.get(play[0])
        b = play[1]

        # lose
        if b == "X":
            outcome = 0
            if a == 1:
                wpn = 3
            elif a == 2:
                wpn = 1
            elif a == 3:
                wpn = 2

        # draw
        elif b == "Y":
            outcome = 3
            wpn = a

        # win
        elif b == "Z":
            outcome = 6
            if a == 1:
                wpn = 2
            elif a == 2:
                wpn = 3
            elif a == 3:
                wpn = 1

        final = final + wpn + outcome

        # print(f"{play[0]} {play[1]} {a}|{b}")


print(final)
