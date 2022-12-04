with open("input") as f:
    elves = []
    curr = 0

    for line in f.readlines():
        line = line.strip()
        if line != "":
            curr += int(line)
        else:
            elves.append(curr)
            curr = 0

    elves.sort()
    print(f"Part 1: {elves[-1]}")
    print(f"Part 2: {sum(elves[-3:])}")
