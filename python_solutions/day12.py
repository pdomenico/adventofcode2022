def get_input(filename):
    with open(filename) as input:
        out = []
        while line := input.readline().rstrip():
            out.append(list(line))
    return out


def get_neighbors(table, start_coord):
    (i, j) = start_coord
    start_char = table[i][j]
    if start_char == "S":
        return [(i - 1, j), (i + 1, j), (i, j + 1)]

    neighbors = []

    if i > 0 and (
        ord(table[i - 1][j]) <= (ord(start_char) + 1) or table[i - 1][j] == "E"
    ):
        neighbors.append((i - 1, j))

    if i < len(table) - 1 and (
        ord(table[i + 1][j]) <= (ord(start_char) + 1) or table[i + 1][j] == "E"
    ):
        neighbors.append((i + 1, j))

    if j > 0 and (
        ord(table[i][j - 1]) <= (ord(start_char) + 1) or table[i][j - 1] == "E"
    ):
        neighbors.append((i, j - 1))

    if j < len(table[0]) - 1 and (
        ord(table[i][j + 1]) <= (ord(start_char) + 1) or table[i][j + 1] == "E"
    ):
        neighbors.append((i, j + 1))

    return neighbors


table = get_input("inputs/day12.txt")
# print(f"Table size: {len(table) * len(table[0])}")
start_coordinates = None
target_coordinates = None
for i in range(len(table)):
    for j in range(len(table[i])):
        if table[i][j] == "S":
            start_coordinates = (i, j)
        if table[i][j] == "E":
            target_coordinates = (i, j + 1)

frontier = set()
frontier.add(start_coordinates)
next_frontier = set()
visited = set()

found = False
steps = 0
while True:
    for coord in frontier:

        if coord == target_coordinates:
            found = True
            steps += 1
            break

        visited.add(coord)

        neighbors = get_neighbors(table, coord)
        for neighbor in neighbors:
            if neighbor not in visited and neighbor not in next_frontier:
                next_frontier.add(neighbor)

    if found:
        break
    frontier = next_frontier.copy()
    next_frontier.clear()
    steps += 1


print("Found in {} steps".format(steps))

starting_points = []
paths = []
for i in range(len(table)):
    for j in range(len(table[i])):
        if table[i][j] == "a":
            starting_points.append((i, j))

for starting_point in starting_points:
    frontier = set()
    frontier.add(starting_point)
    next_frontier = set()
    visited = set()
    found = False
    steps = 0

    while True:
        # print(f"Frontier size: {len(frontier)}")
        for coord in frontier:
            if coord == target_coordinates:
                found = True
                steps += 1
                break

            visited.add(coord)
            neighbors = get_neighbors(table, coord)
            for neighbor in neighbors:
                if not neighbor in visited and not neighbor in next_frontier:
                    next_frontier.add(neighbor)

        if found:
            paths.append(steps)
            break
        if len(next_frontier) == 0:
            break
        frontier = next_frontier.copy()
        next_frontier.clear()
        steps += 1
    steps = 0
    # print(f"Paths: {len(paths)}")

paths.sort()
print(f"Shortest path: {paths[0]}")
