class Monkey:
    def __init__(self, index, items, operation, operation_number, test_divisor, true_monkey_index, false_monkey_index):
        self.index = index
        self.items = items
        self.operation = operation
        self.operation_number = operation_number
        self.test_divisor = test_divisor
        self.true_monkey_index = true_monkey_index
        self.false_monkey_index = false_monkey_index

    def __str__(self):
        return 'Monkey ' + str(self.index) + '\n\tItems: ' + str(self.items) + '\n\tOperation: '\
            + self.operation + '\n\tOperation Number: ' + str(self.operation_number) +\
            '\n\tTest Divisor: ' + str(self.test_divisor) + '\n\tTrue Monkey Index: ' +\
            str(self.true_monkey_index) + \
            '\n\tFalse Monkey Index: ' + str(self.false_monkey_index)


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
                    test_divisor = line.split(' ')[-1]
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

    for monkey in monkeys:
        print(monkey)
