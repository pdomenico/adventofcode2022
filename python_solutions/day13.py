from enum import Enum


class Result(Enum):
    RIGHT = 0
    WRONG = 1
    TIE = 2


def compare(a, b):
    match (a, b):
        case ([a1, *a_rest], [b1, *b_rest]):
            if not (res := compare(a1, b1)) == Result.TIE:
                return res
            return compare(a_rest, b_rest)

        case ([], _):
            return Result.RIGHT

        case (_, []):
            return Result.WRONG

        case ([], []):
            return Result.TIE

        case (_, [_, *_]):
            return compare([a], b)

        case ([_, *_], _):
            return compare(a, [b])

        case _:
            if a < b:
                return Result.RIGHT
            elif a > b:
                return Result.WRONG
            else:
                return Result.TIE


def str_to_list(s: str) -> list[list | int]:
    if s == []:
        return s

    if s.startswith("["):
        s = s[1:]
    if s.endswith("]"):
        s = s[:-1]

    res = []

    i = 0
    while i < len(s):
        if s[i].isdigit():
            start = i
            i += 1
            while i < len(s) and s[i] != ",":
                i += 1
            res.append(int(s[start:i]))
            i += 1
            continue

        if s[i] == "[":
            paren_count = 1
            start = i
            i += 1
            while i < len(s) and paren_count > 0:
                if s[i] == "[":
                    paren_count += 1
                if s[i] == "]":
                    paren_count -= 1
                i += 1
            res.append(str_to_list(s[start:i]))

        i += 1

    return res


pairs = []
with open("inputs/day13.txt") as input:
    while line1 := input.readline().rstrip():
        line2 = input.readline().rstrip()
        pairs.append((str_to_list(line1), str_to_list(line2)))
        input.readline()


score = 0
for (i, (a, b)) in enumerate(pairs):
    if compare(a, b) == Result.RIGHT:
        score += i + 1

print(score)
