import math


class Monkey:
    def __init__(self, index, items, operation, operation_number, test_divisor, true_monkey_index, false_monkey_index):
        self.index = index
        self.items = items
        self.operation = operation
        self.operation_number = operation_number
        self.test_divisor = test_divisor
        self.true_monkey_index = true_monkey_index
        self.false_monkey_index = false_monkey_index
        self.number_of_inspections = 0

    def add_item(self, item):
        self.items.append(item)

    def clear_items(self):
        self.items.clear()

    def do_operation(self, worry_level):
        # if operation number is old, set number to item, else set number to operation number
        if self.operation_number == 'old':
            number = worry_level
        else:
            number = self.operation_number

        # perform operation
        match self.operation:
            case '+':
                result = worry_level + number
            case '-':
                result = worry_level - number
            case '*':
                result = worry_level * number
            case '/':
                result = worry_level / number
            case _:
                print('Cannot perform operation for monkey ' +
                      str(self.index) + ', operation is not valid')
                result = 0

        self.number_of_inspections += 1
        return result

    def get_next_monkey_index(self, worry_level):
        if worry_level % self.test_divisor == 0:
            return self.true_monkey_index
        else:
            return self.false_monkey_index

    def __str__(self):
        return 'Monkey ' + str(self.index) + '\n\tItems: ' + str(self.items) + '\n\tOperation: '\
            + self.operation + '\n\tOperation Number: ' + str(self.operation_number) +\
            '\n\tTest Divisor: ' + str(self.test_divisor) + '\n\tTrue Monkey Index: ' +\
            str(self.true_monkey_index) + \
            '\n\tFalse Monkey Index: ' + \
            str(self.false_monkey_index) + '\n\tNumber of Inspections: ' + \
            str(self.number_of_inspections)


with open('day_11_input.txt') as notes:
    # init variables
    monkeys = []

    line = notes.readline().strip()
    while (line):
        # monkey index
        if line.startswith('Monkey'):
            # get attributes of monkey
            index = int(line.split(' ')[1][:-1])
            line = notes.readline().strip()
            while (line != ''):
                # starting items
                if line.startswith('Starting items:'):
                    items = [int(item.strip())
                             for item in line.split(':')[1].split(',')]
                # operation
                elif line.startswith('Operation'):
                    [operation, operation_number] = line.split(
                        ':')[1].strip().split(' ')[-2:]
                    # if operation number is not old, convert to string
                    if operation_number != 'old':
                        operation_number = int(operation_number)
                # test
                elif line.startswith('Test'):
                    test_divisor = int(line.split(' ')[-1])
                # test true condition
                elif line.startswith('If true'):
                    true_monkey_index = int(line.split(' ')[-1])
                # test false condition
                elif line.startswith('If false'):
                    false_monkey_index = int(line.split(' ')[-1])
                # this should not happen
                else:
                    print('Cannot identify line "' + line +
                          '" for monkey ' + str(index))

                # go to next line
                line = notes.readline().strip()

            # add new monkey to list
            monkeys.append(Monkey(index, items, operation, operation_number,
                           test_divisor, true_monkey_index, false_monkey_index))
            line = notes.readline().strip()

        # here this line should again begin with Monkey or empty
        if not line.startswith('Monkey') and not line == '':
            print('Cannot identify line "' + line + '" at all')

    # iterate over 20 rounds
    for i in range(0, 20):
        # iterate over all monkeys
        for monkey in monkeys:
            for worry_level in monkey.items:
                # operation
                worry_level = monkey.do_operation(worry_level)

                # monkey gets bored
                worry_level = math.floor(worry_level / 3)

                # give item to next monkey
                next_monkey_index = monkey.get_next_monkey_index(worry_level)
                monkeys[next_monkey_index].add_item(worry_level)

            # monkey has no more items
            monkey.clear_items()

    # number of inspections
    number_of_inspections = []
    for monkey in monkeys:
        number_of_inspections.append(monkey.number_of_inspections)

    # answer 1
    print(math.prod(sorted(number_of_inspections, reverse=True)[:2]))
