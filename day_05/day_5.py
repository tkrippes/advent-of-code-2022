with open('day_5_input.txt') as cargo:
    # init variables
    lines = cargo.readlines()
    breaking_line = 0
    stacks_1 = {}
    stacks_2 = {}
    commands = []

    # calculate breaking line
    for line in lines:
        if line == '\n':
            break
        breaking_line += 1

    # init stack keys
    for index in range(1, len(lines[breaking_line - 1]), 4):
        stacks_1[lines[breaking_line - 1][index]] = []
        stacks_2[lines[breaking_line - 1][index]] = []

    # init stacks
    for line in reversed(lines[:breaking_line - 1]):
        for index, index_2 in zip(range(1, len(line), 4), stacks_1.keys()):
            if line[index] != ' ':
                stacks_1[index_2].append(line[index])
                stacks_2[index_2].append(line[index])

    # parse commands
    for line in lines[breaking_line + 1:]:
        commands.append({'quantity': int(line.split(' ')[1]), 'source': line.split(' ')[
                        3], 'destination': line.split(' ')[5].strip()})

    # apply commands
    for command in commands:
        # stack 1
        for i in range(0, command['quantity']):
            cargo = stacks_1[command['source']].pop()
            stacks_1[command['destination']].append(cargo)

        # stack 2
        cargos = stacks_2[command['source']][-command['quantity']:]
        stacks_2[command['source']] = stacks_2[command['source']
                                               ][:-command['quantity']]
        stacks_2[command['destination']] += cargos

    # answer 1
    print(''.join([stacks_1[key][-1] for key in stacks_1.keys()]))

    # answer 2
    print(''.join([stacks_2[key][-1] for key in stacks_2.keys()]))
