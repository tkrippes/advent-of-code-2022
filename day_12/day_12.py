# try to implement the Dijkstra algorithm

with open('day_12_input.txt') as height_map_input:
    # init variables
    lines = height_map_input.readlines()
    start_position = [0, 0]
    current_position = [0, 0]
    end_position = [0, 0]
    height_map = []

    # build height map and initialize important positions
    for line in lines:
        height_map.append([])
        for position in line.strip():
            # start position
            if position == 'S':
                height_map[-1].append(1)
                start_position = [len(height_map) - 1, len(height_map[-1]) - 1]
                current_position = [
                    len(height_map) - 1, len(height_map[-1]) - 1]

            # end position
            elif position == 'E':
                height_map[-1].append(26)
                end_position = [len(height_map) - 1, len(height_map[-1]) - 1]

            # other position
            else:
                height_map[-1].append(ord(position) - ord('a') + 1)

    # TODO remove
    print(start_position)
    print(current_position)
    print(end_position)
    print(height_map)
