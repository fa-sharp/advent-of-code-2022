use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    // create the file and directory structure
    let root_dir = create_files_and_directories(input);

    // create a flattened list of directories with their names and sizes
    let flattened_dirs = root_dir.get_all_dirs();

    // keep the directories with a size <= 100,000, and sum their sizes
    let sum = flattened_dirs
        .filter(|(_, _, size)| size <= &100000)
        .fold(0, |acc, (_, _, size)| acc + size);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // create the file and directory structure
    let root_dir = create_files_and_directories(input);

    // figure out current size and how much space we need
    let current_size = root_dir.calc_size();
    let needed_space = 30000000 - (70000000 - current_size);

    // sort the directories by increasing size, and find the first one >= needed_space
    let mut flattened_dirs: Vec<(&Directory, &str, u32)> = root_dir.get_all_dirs().collect();
    flattened_dirs.sort_by(|(_, _, a_size), (_, _, b_size)| a_size.cmp(b_size));
    let dir_to_delete = flattened_dirs
        .iter()
        .find(|(_, _, size)| *size >= needed_space)
        .expect("No directory found to delete!");

    Some(dir_to_delete.2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}

struct Directory {
    files: HashMap<String, File>,
    directories: HashMap<String, Directory>,
}

impl Directory {
    fn new() -> Directory {
        Self {
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }
    fn calc_size(&self) -> u32 {
        let mut size = 0_u32;
        for (_, file) in &self.files {
            size += file.size;
        }
        for (_, directory) in &self.directories {
            size += directory.calc_size();
        }
        size
    }
    /** Recursively create a flattened list of directories, with their names and sizes */
    fn get_all_dirs(&self) -> Box<dyn Iterator<Item = (&Directory, &str, u32)> + '_> {
        Box::new(
            std::iter::once((self, "self", self.calc_size())).chain(
                self.directories
                    .iter()
                    .map(|(child_dir_name, child_dir)| {
                        (child_dir, child_dir_name, child_dir.calc_size())
                    })
                    .flat_map(|(child_dir, _, _)| child_dir.get_all_dirs()),
            ),
        )
    }
}

#[derive(Debug)]
struct File {
    size: u32,
}

impl File {
    fn new(size: u32) -> Self {
        Self { size }
    }
}

enum Command {
    RootDir,
    OpenDir { dir_name: String },
    UpDir,
    ListFiles,
}

enum FileOrDir {
    File { file: File, name: String },
    Dir,
}

/** Parse command: '$ cd foo', etc. */
fn parse_command(raw_str: &str) -> Option<Command> {
    match raw_str.get(0..4) {
        Some("$ cd") => {
            let dir_name = raw_str.strip_prefix("$ cd ").unwrap();
            if dir_name == "/" {
                Some(Command::RootDir)
            } else if dir_name == ".." {
                Some(Command::UpDir)
            } else {
                Some(Command::OpenDir {
                    dir_name: dir_name.to_owned(),
                })
            }
        }
        Some("$ ls") => Some(Command::ListFiles),
        _ => None,
    }
}

/** Parse file or directory name: 'dir foo' or '300 foo.txt' */
fn parse_file_or_dir(raw_str: &str) -> Option<FileOrDir> {
    if let Some(_) = raw_str.strip_prefix("dir ") {
        return Some(FileOrDir::Dir);
    } else if let Some((raw_size, file_name)) = raw_str.split_once(' ') {
        let size: u32 = raw_size.parse().expect("Couldn't parse file size!");
        return Some(FileOrDir::File {
            file: File::new(size),
            name: file_name.to_owned(),
        });
    }
    None
}

/** Create the file and directory structure from the input file, and return the root folder */
fn create_files_and_directories(input: &str) -> Directory {
    // Stacks to keep track of where we are in the directory structure
    let mut stack_dirs: Vec<Directory> = vec![Directory::new()];
    let mut stack_names: Vec<String> = vec![String::from("/")];

    for raw_str in input.lines() {
        // check if this line is a command
        match parse_command(raw_str) {
            Some(Command::RootDir) => {
                while stack_dirs.len() > 1 {
                    stack_dirs.pop();
                }
                stack_names = vec![String::from("/")];
            }
            Some(Command::UpDir) => {
                let current_dir = stack_dirs.pop().expect("Current directory not found!");
                let current_dir_name = stack_names.pop().unwrap();
                stack_dirs
                    .last_mut()
                    .expect("Parent directory not found!")
                    .directories
                    .entry(current_dir_name)
                    .or_insert(current_dir); // Would need to add merging here!
            }
            Some(Command::OpenDir { dir_name }) => {
                stack_dirs.push(Directory::new());
                stack_names.push(dir_name);
            }
            Some(Command::ListFiles) => (),

            // if line is not a command, parse and insert the files and directories in the current directory
            _ => match parse_file_or_dir(raw_str) {
                Some(FileOrDir::File { file, name }) => {
                    let current_dir = stack_dirs.last_mut().expect("Current directory not found!");
                    current_dir.files.insert(name, file);
                }
                Some(FileOrDir::Dir) => (), // ignore directories as we're adding them above
                None => panic!("Couldn't detect command or file/directory!"),
            },
        }
    }

    while stack_dirs.len() > 1 {
        let current_dir = stack_dirs.pop().expect("Current directory not found!");
        let current_dir_name = stack_names.pop().unwrap();
        stack_dirs
            .last_mut()
            .expect("Parent directory not found!")
            .directories
            .entry(current_dir_name)
            .or_insert(current_dir); // Would need to add merging here!
    }
    stack_dirs.pop().unwrap()
}
