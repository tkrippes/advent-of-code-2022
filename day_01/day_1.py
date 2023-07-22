with open('day_1_input.txt') as calories:
    # init variables
    lines = calories.readlines()
    elf_number = 1
    elf_count = {elf_number: 0}

    # add calories for each elf
    for line in lines:
        if (line != '\n'):
            elf_count[elf_number] += int(line)
        else:
            elf_number += 1
            elf_count[elf_number] = 0

# answer 1
print(max(elf_count.values()))

# answer 2
print(sum(sorted(elf_count.values(), reverse=True)[:3]))
