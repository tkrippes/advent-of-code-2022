def add_directory_to_file_system(file_system, current_path, directory_name):
    # go to directory where new directory should be added
    tmp_directory = file_system
    for directory in current_path:
        tmp_directory = tmp_directory[directory]

    # add new empty directory
    tmp_directory[directory_name] = {}
    return file_system


def add_file_to_file_system(file_system, current_path, file_size, file_name):
    # # go to directory where new file should be added
    tmp_directory = file_system
    for directory in current_path:
        tmp_directory = tmp_directory[directory]

    # add new file
    tmp_directory[file_name] = file_size
    return file_system


def update_current_path(current_path, directory):
    # go back to root
    if directory == '/':
        current_path = ['/']
    # go back one folder
    elif directory == '..':
        current_path = current_path[:-1]
    # go into folder
    else:
        current_path.append(directory)

    return current_path


with open('day_7_input.txt') as command_line:
    # init variables
    file_system = {'/': {}}
    current_path = ['/']

    # calculate file system
    line = command_line.readline().strip()
    while (line):
        # line is a command
        if line.startswith('$'):
            command = line.split(' ')[1]

            # change directory command
            if command == 'cd':
                directory = line.split(' ')[2]
                current_path = update_current_path(current_path, directory)
                line = command_line.readline().strip()

            # list command
            elif command == 'ls':
                line = command_line.readline().strip()

                # read lines until another command is found
                while not line.startswith('$') and line:
                    # add directory to file system
                    if line.startswith('dir'):
                        directory = line.split(' ')[1]
                        file_system = add_directory_to_file_system(
                            file_system, current_path, directory)

                    # add file to file system
                    else:
                        [file_size, file_name] = line.split(' ')
                        file_size = int(file_size)
                        file_system = add_file_to_file_system(
                            file_system, current_path, file_size, file_name)
                    line = command_line.readline().strip()

        else:
            print('This should not happen ..., aborting')
            break

    # answer 1 TODO
    print(file_system)

    # answer 2
