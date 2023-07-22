# Opponent: A: rock, B: paper, C: scissors
# You (1): X: rock, Y: paper, Z: scissors
# Outcome (2): X: loss, Y: draw, Z: win
# Points: rock: 1, paper: 2, scissors: 3
# loss: 0, draw: 3, win: 6

import os

file_name = os.path.join('..', 'input', 'input.txt')

def points_for_match_1(opponent_shape, your_shape):
    match opponent_shape:
        case 'A':
            match your_shape:
                case 'X':
                    return 3 + 1
                case 'Y':
                    return 6 + 2
                case 'Z':
                    return 0 + 3
                case _:
                    return 0
        case 'B':
            match your_shape:
                case 'X':
                    return 0 + 1
                case 'Y':
                    return 3 + 2
                case 'Z':
                    return 6 + 3
                case _:
                    return 0
        case 'C':
            match your_shape:
                case 'X':
                    return 6 + 1
                case 'Y':
                    return 2
                case 'Z':
                    return 3 + 3
                case _:
                    return 0
        case _:
            return 0


def points_for_match_2(opponent_shape, out_come):
    match opponent_shape:
        case 'A':
            match out_come:
                case 'X':
                    return 0 + 3
                case 'Y':
                    return 3 + 1
                case 'Z':
                    return 6 + 2
                case _:
                    return 0
        case 'B':
            match out_come:
                case 'X':
                    return 0 + 1
                case 'Y':
                    return 3 + 2
                case 'Z':
                    return 6 + 3
                case _:
                    return 0
        case 'C':
            match out_come:
                case 'X':
                    return 0 + 2
                case 'Y':
                    return 3 + 3
                case 'Z':
                    return 6 + 1
                case _:
                    return 0
        case _:
            return 0


with open(file_name) as strategy:
    # init variables
    lines = strategy.readlines()
    score_1 = 0
    score_2 = 0

    # calculate score
    for line in lines:
        opponent_shape = line[0]
        your_shape = line[2]
        outcome = line[2]

        score_1 += points_for_match_1(opponent_shape, your_shape)
        score_2 += points_for_match_2(opponent_shape, outcome)

    # answer 1
    print(score_1)

    # answer 2
    print(score_2)
