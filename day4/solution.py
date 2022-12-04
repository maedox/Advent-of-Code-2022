part1_score = 0
part2_score = 0

with open("input") as f:
    for line in f.readlines():
        first, last = line.split(",")
        first_start, first_end = [int(i) for i in first.split("-")]
        last_start, last_end = [int(i) for i in last.split("-")]

        first_range = range(first_start, first_end + 1)
        last_range = range(last_start, last_end + 1)

        if (last_start in first_range and last_end in first_range) or (
            first_start in last_range and first_end in last_range
        ):
            part1_score += 1
        if (
            last_start in first_range
            or last_end in first_range
            or first_start in last_range
            or first_end in last_range
        ):
            part2_score += 1

    print(f"Part 1: {part1_score}")
    print(f"Part 2: {part2_score}")
