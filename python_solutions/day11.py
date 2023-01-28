import sys
import math


class Monkey:
    def __init__(self, items, operator, operand, divisor, choice1, choice2):
        self.items = items
        self.score = 0
        match operator:
            case "*":
                if operand.isnumeric():
                    self.operation = lambda x: x * int(operand)
                else:
                    self.operation = lambda x: x * x
            case _:
                if operand.isnumeric():
                    self.operation = lambda x: x + int(operand)
                else:
                    self.operation = lambda x: x + x

        self.test = lambda x: choice1 if x % divisor == 0 else choice2

    def turn(self, monkeys):
        for item in self.items:
            new_val = self.operation(item) % 9699690
            destination = self.test(new_val)
            monkeys[destination].add_item(new_val)
            self.score += 1
        self.items = []

    def add_item(self, item):
        self.items.append(item)


monkeys = []

with open("../inputs/day11.txt") as input:
    while line := input.readline().rstrip():
        items = input.readline().rstrip().split()[2:]
        items = [int(x.replace(",", "")) for x in items]

        operation_line = input.readline().rstrip().split()[-2:]
        operator = operation_line[0]
        operand = operation_line[1]

        divisor = int(input.readline().rstrip().split()[-1])
        monkey1 = int(input.readline().rstrip().split()[-1])
        monkey2 = int(input.readline().rstrip().split()[-1])

        monkeys.append(Monkey(items, operator, operand, divisor, monkey1, monkey2))

        input.readline()

for rounds in range(10000):
    print(f"Round {rounds}", end="\r")
    sys.stdout.flush()
    for monkey in monkeys:
        monkey.turn(monkeys)

scores = [monkey.score for monkey in monkeys]
scores.sort()
scores.reverse()
print(scores[0] * scores[1])
