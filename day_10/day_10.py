with open('day_10_input.txt') as instructions:
    # init variables
    registry_value = 1
    cycle_number = 1
    waiting_cycle = False
    number = 0
    interested_cycle_numbers = [20, 60, 100, 140, 180, 220]
    signal_strengths = {}
    for interesting_cycle_number in interested_cycle_numbers:
        signal_strengths[interesting_cycle_number] = 0
    crt_width = 40
    crt_sprite = {1: '#'}

    line = instructions.readline().strip()
    while (line):
        cycle_number += 1

        # last cycle was a waiting cycle, apply addition now
        if waiting_cycle:
            waiting_cycle = False
            registry_value += number
            line = instructions.readline().strip()

        # last cycle was not waiting cycle, read instructions
        else:
            # read instruction
            instruction = line.strip().split(' ')

            # noop command, do nothing
            if instruction[0] == 'noop':
                line = instructions.readline().strip()

            # add command, add number to registry after waiting
            elif instruction[0] == 'addx':
                number = int(instruction[1])
                waiting_cycle = True

        # check for interesting cycle number
        if cycle_number in interested_cycle_numbers:
            signal_strengths[cycle_number] = cycle_number * registry_value

        # set crt output
        if (cycle_number - 1) % crt_width in range(registry_value - 1, registry_value + 2):
            crt_sprite[cycle_number] = '#'
        else:
            crt_sprite[cycle_number] = '.'

    # answer 1
    print(sum(signal_strengths.values()))

    # answer 2
    result_2 = ''
    for i in range(0, 6):
        print(''.join(crt_sprite.values())[i*crt_width:(i+1)*crt_width])
