def compare(a, b):
    match (a, b):
        case ([a1, *_], [b1, *others]):
            print("Two non empty lists")
            print(others)
        case ([], [b1, *_]):
            print("One empty one non empty")
        case (a, [b, *_]):
            print("One element, one list")
        case _:
            print("Something else")


compare(2, [2])
