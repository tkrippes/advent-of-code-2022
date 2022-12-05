def intersection(list_1, list_2):
    return [element for element in list_1 if element in list_2]


with open('day_4_input.txt') as workload:
    # init variables
    lines = workload.readlines()
    number_of_whole_overlaps = 0
    number_of_partial_overlaps = 0

    # calculate score
    for line in lines:
        work_intervals = line.split(',')
        work_limits_1 = work_intervals[0].split('-')
        work_limits_2 = work_intervals[1].split('-')
        work_range_1 = [i for i in range(
            int(work_limits_1[0]), int(work_limits_1[1]) + 1)]
        work_range_2 = [i for i in range(
            int(work_limits_2[0]), int(work_limits_2[1]) + 1)]

        if len(intersection(work_range_1, work_range_2)) == min(len(work_range_1), len(work_range_2)):
            number_of_whole_overlaps += 1

        if len(intersection(work_range_1, work_range_2)) >= 1:
            number_of_partial_overlaps += 1

    # answer 1
    print(number_of_whole_overlaps)

    # answer 2
    print(number_of_partial_overlaps)
