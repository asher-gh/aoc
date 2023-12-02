# the With statement in Python is used in exception handling
# to make code cleaner and more readable. Otherwise, everytime
# a file is opened it need to


calories: list[int] = []


with open("inputs/d1") as f:
    count = 0
    sum = 0
    lines = [line.strip() for line in f]

    for n in lines:
        if n == "":
            count = 0
            calories.append(sum)
            sum = 0
        else:
            sum += int(n)

    calories.append(sum)

sum = 0
calories = sorted(calories, reverse=True)

for i in range(0, 3):
    sum += calories[i]

print(sum)
