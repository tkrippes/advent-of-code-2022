def get_shared_character_2(string_1, string_2):
    # search for common character
    for character in string_1:
        if character in string_2:
            return character
    
    # if no common character, return empty string
    return ''
    
def get_shared_character_3(string_1, string_2, string_3):
    # search for common character
    for character in string_1:
        if character in string_2 and character in string_3:
            return character
    
    # if no common character, return empty string
    return ''

def priority_of_character(character):
    # lowercase letter
    if ord(character) >= ord('a') and ord(character) <= ord('z'):
        return ord(character) - ord('a') + 1
    # uppercase letter
    elif ord(character) >= ord('A') and ord(character) <= ord('Z'):
        return ord(character) - ord('A') + 27
    else:
        return 0

with open('data_3_input.txt') as rucksacks:
    # init variables
    lines = rucksacks.readlines()
    sum_of_priorities = 0
    
    # calculate sum of priorities  1
    for line in lines:
        # strip any leading or trailing whitespaces
        line = line.strip()
        first_compartment_items = line[:len(line) // 2]
        second_compartment_items = line[len(line) // 2:]
        character = get_shared_character_2(first_compartment_items, second_compartment_items)
        sum_of_priorities += priority_of_character(character)
        
    # result 1
    print(sum_of_priorities)
