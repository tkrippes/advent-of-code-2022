import os

file_name = os.path.join('..', 'input', 'input.txt')

def get_index_of_first_different_characters(input, number_of_characters):
    # starting point
    for i in range(0, len(input) - number_of_characters):
        only_unique_characters = True
        # check
        for j in range(i, i + number_of_characters):
            for k in range(i + 1, i + number_of_characters):
                # skip for same index
                if (j == k):
                    continue
                # if same characters were found, break (go to next i)
                if input[j] == input[k]:
                    only_unique_characters = False
                    break
            # if same characters were found, break (go to next i)
            if not only_unique_characters:
                break

        # if all characters were different, index was found
        if only_unique_characters:
            return i

    # default -1
    return -1


with open(file_name) as datastream:
    # init variables
    data = datastream.readlines()

    # calculate index of first four different characters
    index_of_first_four_different_characters = get_index_of_first_different_characters(
        data[0], 4)
    index_of_first_fourteen_different_characters = get_index_of_first_different_characters(
        data[0], 14)

    # answer 1
    print(index_of_first_four_different_characters + 4)

    # answer 2
    print(index_of_first_fourteen_different_characters + 14)
