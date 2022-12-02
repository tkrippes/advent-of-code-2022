# Opponent: A: rock, B: paper, C: scissors
# You: X: rock, Y: paper, Z: scissors

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


with open('day_2_input.txt') as strategy:
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
