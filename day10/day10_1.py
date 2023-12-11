from collections import deque

grid = open("d10input").read().strip().splitlines()

for sr, row in enumerate(grid):
    for sc, c in enumerate(row):
        if c == "S":
            break
    else:
        continue
    break

seen = {(sr, sc)}
q = deque([(sr, sc)])

while q:
    r, c = q.popleft()
    ch = grid[r][c]

    if r > 0 and ch in "S|JL" and grid[r - 1][c] in "|7F" and (r - 1, c) not in seen:
        seen.add((r - 1, c))
        q.append((r - 1, c))

    if (
        r < len(grid) - 1
        and ch in "S|7F"
        and grid[r + 1][c] in "|JL"
        and (r + 1, c) not in seen
    ):
        seen.add((r + 1, c))
        q.append((r + 1, c))

    if c > 0 and ch in "S-J7" and grid[r][c - 1] in "-FL" and (r, c - 1) not in seen:
        seen.add((r, c - 1))
        q.append((r, c - 1))

    if (
        c < len(grid[r]) - 1
        and ch in "S-FL"
        and grid[r][c + 1] in "-J7"
        and (r, c + 1) not in seen
    ):
        seen.add((r, c + 1))
        q.append((r, c + 1))
