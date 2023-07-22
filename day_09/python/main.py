import os

file_name = os.path.join('..', 'input', 'input.txt')

def add_tail_position_if_not_present(visited_positions, tail_position):
    found_position = False

    # search for tail position in visited position
    for position in visited_positions:
        if position[0] == tail_position[0] and position[1] == tail_position[1]:
            found_position = True
            break

    # only add tail position if not present in visited positions
    if not found_position:
        visited_positions.append(tail_position.copy())

    return visited_positions


def get_distance_between_positions(leading_element_position, current_element_position):
    # head and tail on same position
    if leading_element_position == current_element_position:
        return 0

    # head an tail on
    if abs(leading_element_position[0] - current_element_position[0]) <= 1 and abs(leading_element_position[1] - current_element_position[1]) <= 1:
        return 1

    # if distance between head and tail is bigger than 1, assume 2
    return 2


def move_element(leading_element_position, current_element_position):
    # init variables
    x_difference = leading_element_position[0] - current_element_position[0]
    y_difference = leading_element_position[1] - current_element_position[1]

    # move up or down
    if x_difference == 0:
        # move down
        if y_difference < 0:
            current_element_position[1] -= 1
        # move up
        elif y_difference > 0:
            current_element_position[1] += 1
        return current_element_position

    # move left or right
    if y_difference == 0:
        # move left
        if x_difference < 0:
            current_element_position[0] -= 1
        # move right
        elif x_difference > 0:
            current_element_position[0] += 1
        return current_element_position

    # move diagonally
    if x_difference < 0 and y_difference < 0:
        current_element_position[0] -= 1
        current_element_position[1] -= 1
    elif x_difference < 0 and y_difference > 0:
        current_element_position[0] -= 1
        current_element_position[1] += 1
    elif x_difference > 0 and y_difference < 0:
        current_element_position[0] += 1
        current_element_position[1] -= 1
    elif x_difference > 0 and y_difference > 0:
        current_element_position[0] += 1
        current_element_position[1] += 1

    return current_element_position


with open(file_name) as movements:
    # init variables
    lines = movements.readlines()
    head_position = [0, 0]
    tail_position = [0, 0]
    visited_positions_1 = []
    visited_positions_1 = add_tail_position_if_not_present(
        visited_positions_1, tail_position)

    tail_positions = [[0, 0] for i in range(0, 9)]
    visited_positions_2 = []
    visited_positions_2 = add_tail_position_if_not_present(
        visited_positions_2, tail_positions[-1])

    for line in lines:
        [direction, steps] = line.split(' ')
        for i in range(0, int(steps)):
            match direction:
                case 'U':
                    head_position = [
                        head_position[0], head_position[1] + 1]
                case 'D':
                    head_position = [
                        head_position[0], head_position[1] - 1]
                case 'L':
                    head_position = [
                        head_position[0] - 1, head_position[1]]
                case 'R':
                    head_position = [
                        head_position[0] + 1, head_position[1]]

            if get_distance_between_positions(head_position, tail_position) > 1:
                tail_position = move_element(
                    head_position, tail_position)

            for i in range(0, len(tail_positions)):
                # first tail element follows head
                if i == 0:
                    if get_distance_between_positions(head_position, tail_positions[i]) > 1:
                        tail_positions[i] = move_element(
                            head_position, tail_positions[i])
                # other tail elements follow the preceding tail element
                else:
                    if get_distance_between_positions(tail_positions[i - 1], tail_positions[i]) > 1:
                        tail_positions[i] = move_element(
                            tail_positions[i - 1], tail_positions[i])

            visited_positions_1 = add_tail_position_if_not_present(
                visited_positions_1, tail_position)

            visited_positions_2 = add_tail_position_if_not_present(
                visited_positions_2, tail_positions[-1])

    # answer 1
    print(len(visited_positions_1))

    # answer 2
    print(len(visited_positions_2))
