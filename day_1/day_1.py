with open('calories_input.txt') as calories:
    # init variables
    lines = calories.readlines()
    elf_number = 1
    elf_count = {elf_number: 0}

    for line in lines:
        if (line != '\n'):
            elf_count[elf_number] += int(line)
        else:
            elf_number += 1
            elf_count[elf_number] = 0

print(max(elf_count.values()))
