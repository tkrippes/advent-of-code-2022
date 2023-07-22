import os

file_name = os.path.join('..', 'input', 'input.txt')

with open(file_name) as tree_map_input:
    # init variables
    lines = tree_map_input.readlines()
    tree_map = []
    number_of_visible_trees = 0
    scenic_score = []

    # fill tree map with heights
    for line in lines:
        line = line.strip()
        tree_map.append([])
        for height in line:
            tree_map[-1].append(int(height))

    # add outer visible trees
    number_of_visible_trees += 2 * len(tree_map) + 2 * (len(tree_map[0]) - 2)

    # check for inner visible trees
    for i in range(1, len(tree_map) - 1):
        for j in range(1, len(tree_map[0]) - 1):
            tree_visible = True

            # check for top
            for k in range(0, i):
                if tree_map[k][j] >= tree_map[i][j]:
                    tree_visible = False
                    break

            # tree visible from top
            if tree_visible:
                number_of_visible_trees += 1
                continue

            tree_visible = True

            # check for bottom
            for k in range(i + 1, len(tree_map)):
                if tree_map[k][j] >= tree_map[i][j]:
                    tree_visible = False
                    break

            # tree visible from botton
            if tree_visible:
                number_of_visible_trees += 1
                continue

            tree_visible = True

            # check for left
            for k in range(0, j):
                if tree_map[i][k] >= tree_map[i][j]:
                    tree_visible = False
                    break

            # tree visible from left
            if tree_visible:
                number_of_visible_trees += 1
                continue

            tree_visible = True

            # check for right
            for k in range(j + 1, len(tree_map[0])):
                if tree_map[i][k] >= tree_map[i][j]:
                    tree_visible = False
                    break

            # tree visible from right
            if tree_visible:
                number_of_visible_trees += 1

    # calculate scenic score
    for i in range(0, len(tree_map)):
        scenic_score.append([])
        for j in range(0, len(tree_map[0])):
            scenic_score[-1].append(1)

            # TODO remove
            text = 'Local score for (' + str(i) + ', ' + str(j) + '): '

            # calculate top score (start from tree not from edge)
            local_score = 0
            for k in reversed(range(0, i)):
                local_score += 1
                if tree_map[k][j] >= tree_map[i][j]:
                    break

            # update scenic score
            scenic_score[-1][-1] *= local_score

            # check bottom score
            local_score = 0
            for k in range(i + 1, len(tree_map)):
                local_score += 1
                if tree_map[k][j] >= tree_map[i][j]:
                    break

            # update scenic score
            scenic_score[-1][-1] *= local_score

            # calculate left score (start from tree not from edge)
            local_score = 0
            for k in reversed(range(0, j)):
                local_score += 1
                if tree_map[i][k] >= tree_map[i][j]:
                    break

            # update scenic score
            scenic_score[-1][-1] *= local_score

            # calculate right score
            local_score = 0
            for k in range(j + 1, len(tree_map[0])):
                local_score += 1
                if tree_map[i][k] >= tree_map[i][j]:
                    break

            # update scenic score
            scenic_score[-1][-1] *= local_score

    # answer 1
    print(number_of_visible_trees)

    # answer 2
    print(max(map(max, scenic_score)))
