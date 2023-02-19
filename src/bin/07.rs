use std::{collections::HashMap, rc::Rc, cell::RefCell, fmt};

struct Directory {
    name: String,
    contents: HashMap<String, DirContent>,
    parent: Option<Rc<RefCell<Directory>>>
}
impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_name: String;
        f.debug_struct("Directory")
            .field("name", &self.name)
            .field("contents", &self.contents)
            .field("parent", match &self.parent {
                None => &"No parent (root directory)",
                Some(dir) => {
                    parent_name = dir.borrow().name.clone();
                    &parent_name
                },
            })
            .finish()
    }
}
impl Directory {
    fn new_root(name: &str) -> Self { Self { name: name.to_owned(), contents: HashMap::new(), parent: None } }
    fn new(name: &str, parent: Rc<RefCell<Directory>>) -> Self { Self { name: name.to_owned(), contents: HashMap::new(), 
        parent: Some(parent) } }
    fn add_item(&mut self, item: DirContent) {
        match item {
            DirContent::Directory(dir) => {
                self.contents.insert(dir.name.clone(), DirContent::Directory(dir));
            },
            DirContent::File(file) => {
                self.contents.insert(file.name.clone(), DirContent::File(file));
            }
        }
    }
    fn calc_size(&self) -> u32 {
        let mut size = 0_u32;
        for (_, item) in &self.contents {
            size += match item {
                DirContent::Directory(dir) => dir.calc_size(),
                DirContent::File(file) => file.size
            }
        }
        size
    }
}
#[derive(Debug)]
struct File {
    name: String,
    size: u32
}
impl File {
    fn new(name: &str, size: u32) -> Self { Self { name: name.to_owned(), size } }
}
#[derive(Debug)]
enum DirContent {
    File(File),
    Directory(Directory)
}

pub fn part_one(input: &str) -> Option<u32> {
    let root_dir = Rc::new(RefCell::new(Directory::new_root("/")));
    root_dir.borrow_mut().add_item(DirContent::File(File::new("root.file", 23)));
    let mut a_dir = Directory::new("a", root_dir.clone());
    a_dir.add_item(DirContent::File(File::new("a.1", 23)));
    a_dir.add_item(DirContent::File(File::new("a.2", 23)));
    println!("Size of a dir: {}", a_dir.calc_size());
    
    root_dir.borrow_mut().add_item(DirContent::Directory(a_dir));

    println!("{:#?}", root_dir);
    println!("Size of root dir: {}", root_dir.borrow().calc_size());

    None
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
