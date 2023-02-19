use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

pub fn part_one(input: &str) -> Option<u32> {
    let root_dir = Directory::new_root();

    // create the file and directory structure
    let mut current_dir = root_dir.clone();
    for raw_str in input.lines() {
        // check if this line is a command
        match parse_command(raw_str) {
            Some(Command::RootDir) => current_dir = root_dir.clone(),
            Some(Command::UpDir) => {
                current_dir = current_dir
                    .clone()
                    .borrow()
                    .parent
                    .clone()
                    .expect("No parent directory found!")
            }
            Some(Command::OpenDir { dir_name }) => {
                current_dir = current_dir
                    .clone()
                    .borrow_mut()
                    .directories
                    .entry(dir_name)
                    .or_insert(Directory::new(current_dir.clone()))
                    .clone()
            }
            Some(Command::ListFiles) => (),

            // if line is not a command, parse and insert the files and directories in the current directory
            _ => match parse_file_or_dir(raw_str, current_dir.clone()) {
                Some(FileOrDir::File { file, name }) => {
                    current_dir.borrow_mut().files.insert(name, file);
                }
                Some(FileOrDir::Dir { dir, name }) => {
                    current_dir.borrow_mut().directories.insert(name, dir);
                }
                None => panic!("Couldn't detect command or file/directory!"),
            },
        }
    }

    // create a flattened list of directories with their names and sizes
    let mut flattened_dirs: Vec<(String, u32, Rc<RefCell<Directory>>)> = vec![];
    collect_all_directories(&mut flattened_dirs, root_dir.clone());

    // keep the directories <= 100,000, and sum their sizes
    flattened_dirs.retain(|(_, size, _)| size <= &100000);
    let sum = flattened_dirs
        .iter()
        .fold(0, |acc, (_, size, _)| acc + size);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}

struct Directory {
    files: HashMap<String, File>,
    directories: HashMap<String, Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}
impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Directory")
            .field("files", &self.files)
            .field("directories", &self.directories)
            .finish()
    }
}
impl Directory {
    fn new_root() -> Rc<RefCell<Directory>> {
        Rc::new(RefCell::new(Self {
            files: HashMap::new(),
            directories: HashMap::new(),
            parent: None,
        }))
    }
    fn new(parent: Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>> {
        Rc::new(RefCell::new(Self {
            files: HashMap::new(),
            directories: HashMap::new(),
            parent: Some(parent),
        }))
    }
    fn calc_size(&self) -> u32 {
        let mut size = 0_u32;
        for (_, file) in &self.files {
            size += file.size;
        }
        for (_, directory) in &self.directories {
            size += directory.borrow().calc_size();
        }
        size
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

enum FileOrDir {
    File {
        file: File,
        name: String,
    },
    Dir {
        dir: Rc<RefCell<Directory>>,
        name: String,
    },
}

/** Parse file or directory name: 'dir foo' or '300 foo.txt' */
fn parse_file_or_dir(raw_str: &str, parent_dir: Rc<RefCell<Directory>>) -> Option<FileOrDir> {
    if let Some(dir_name) = raw_str.strip_prefix("dir ") {
        return Some(FileOrDir::Dir {
            dir: Directory::new(parent_dir),
            name: dir_name.to_owned(),
        });
    } else if let Some((raw_size, file_name)) = raw_str.split_once(' ') {
        let size: u32 = raw_size.parse().expect("Couldn't parse file size!");
        return Some(FileOrDir::File {
            file: File::new(size),
            name: file_name.to_owned(),
        });
    }
    None
}

/** Recursively create a flattened list of directories, with their names and sizes */
fn collect_all_directories(
    flattened_dirs: &mut Vec<(String, u32, Rc<RefCell<Directory>>)>,
    current_dir: Rc<RefCell<Directory>>,
) {
    flattened_dirs.append(
        &mut current_dir
            .borrow()
            .directories
            .iter()
            .map(|(dir_name, dir)| (dir_name.to_owned(), dir.borrow().calc_size(), dir.clone()))
            .collect(),
    );
    for (_, child_dir) in &current_dir.borrow().directories {
        collect_all_directories(flattened_dirs, child_dir.clone());
    }
}
