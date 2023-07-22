import os

file_name = os.path.join('..', 'input', 'input.txt')

class File:
    def __init__(self, size, name):
        self.size = size
        self.name = name

    def get_size(self):
        return self.size

    def get_name(self):
        return self.name


class Directory:
    def __init__(self, name):
        self.name = name
        self.size = 0
        self.files = []
        self.sub_directories = []

    def add_file(self, file_size, file_name, relative_path):
        # right relative path
        if relative_path[0] == self.name:
            # file should be inserted here
            if len(relative_path) == 1:
                self.files.append(File(file_size, file_name))
            # file should be inserted in subdirectory structure
            else:
                for subdirectory in self.sub_directories:
                    # if right subdirectory is found, try to insert file here
                    if relative_path[1] == subdirectory.get_name():
                        subdirectory.add_file(
                            file_size, file_name, relative_path[1:])

        # wrong relative path
        else:
            print('This should not happen ... file not inserted')

    def add_subdirectory(self, directory_name, relative_path):
        # right relative path
        if relative_path[0] == self.name:
            # directory should be inserted here
            if len(relative_path) == 1:
                self.sub_directories.append(Directory(directory_name))
            # directory should be inserted in subdirectory structure
            else:
                for subdirectory in self.sub_directories:
                    # if right subdirectory is found, try to insert directory here
                    if relative_path[1] == subdirectory.get_name():
                        subdirectory.add_subdirectory(
                            directory_name, relative_path[1:])

        # wrong relative path
        else:
            print('This should not happen ... directory not inserted')

    def get_name(self):
        return self.name

    def get_size(self):
        return self.size

    def get_directory_sizes(self, path):
        sizes = {}
        path += '/' + self.name
        for sub_directory in self.sub_directories:
            sizes = {
                **sizes, **sub_directory.get_directory_sizes(path)}
        return {**sizes, **{path: self.size}}

    def calculate_size(self):
        # reset previously calculated size
        self.size = 0

        # add sizes of own files
        for file in self.files:
            self.size += file.get_size()

        # add sizes of subdirectories
        for sub_directory in self.sub_directories:
            sub_directory.calculate_size()
            self.size += sub_directory.get_size()

    def toString(self):
        text = self.name + ': { '
        for file in self.files:
            text += file.get_name() + ', '
        for subdirectory in self.sub_directories:
            text += subdirectory.toString()
        text += ' } '
        return text


class FileSystem:
    def __init__(self):
        self.root_directory = Directory('/')

    def add_directory(self, name, relative_path):
        self.root_directory.add_subdirectory(name, relative_path)

    def add_file(self, file_size, file_name, relative_path):
        self.root_directory.add_file(file_size, file_name, relative_path)

    def get_directory_sizes(self):
        self.root_directory.calculate_size()
        return self.root_directory.get_directory_sizes('')

    def __str__(self):
        return self.root_directory.toString()


def update_current_path(current_path, directory):
    # go back to root
    if directory == '/':
        current_path = ['/']
    # go back one directory
    elif directory == '..':
        current_path = current_path[:-1]
    # go into directory
    else:
        current_path.append(directory)

    return current_path


with open(file_name) as command_line:
    # init variables
    file_system = FileSystem()
    current_path = ['/']

    # calculate file system
    line = command_line.readline().strip()
    while (line):
        # line is a command
        if line.startswith('$'):
            command = line.split(' ')[1]

            # change directory command
            if command == 'cd':
                directory_name = line.split(' ')[2]
                current_path = update_current_path(
                    current_path, directory_name)
                line = command_line.readline().strip()

            # list command
            elif command == 'ls':
                line = command_line.readline().strip()

                # read lines until another command is found
                while not line.startswith('$') and line:
                    # add directory to file system
                    if line.startswith('dir'):
                        directory_name = line.split(' ')[1]
                        file_system.add_directory(directory_name, current_path)

                    # add file to file system
                    else:
                        [file_size, file_name] = line.split(' ')
                        file_size = int(file_size)
                        file_system.add_file(
                            file_size, file_name, current_path)
                    line = command_line.readline().strip()

        else:
            print('This should not happen ..., aborting reading input!')
            break

    # calculate directory sizes
    directory_sizes = file_system.get_directory_sizes()

    # calculate the sum of all sizes with at most 100.000
    result_1 = 0
    for size in directory_sizes.values():
        if size < 100000:
            result_1 += size

    # answer 1
    print(result_1)

    # given sizes
    total_disk_space = 70000000
    wanted_unused_disk_space = 30000000

    # calculate disk space to delete
    wanted_used_disk_space = total_disk_space - wanted_unused_disk_space
    used_disk_space = directory_sizes['//']
    disk_space_to_delete = used_disk_space - wanted_used_disk_space

    # search for smallest directory to delete in order to have the wanted unused disk space
    directory_to_delete = '//'
    for directory, size in directory_sizes.items():
        if directory_sizes[directory] > disk_space_to_delete and directory_sizes[directory] < directory_sizes[directory_to_delete]:
            directory_to_delete = directory
    result_2 = directory_sizes[directory_to_delete]

    # answer 2
    print(result_2)
