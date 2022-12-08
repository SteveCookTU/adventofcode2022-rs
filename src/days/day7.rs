use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

struct DirectoryTree<'a> {
    root: Rc<RefCell<DirectoryNode<'a>>>,
}

#[derive(Clone)]
struct DirectoryNode<'a> {
    parent: Option<Rc<RefCell<DirectoryNode<'a>>>>,
    name: &'a str,
    files: Vec<File>,
    directories: Vec<Rc<RefCell<DirectoryNode<'a>>>>,
}

#[derive(Clone)]
struct File {
    size: u64,
}

impl<'a> DirectoryTree<'a> {
    fn find_sizes(
        &self,
        node: Option<Rc<RefCell<DirectoryNode<'a>>>>,
        result: &mut Vec<u64>,
    ) -> u64 {
        let node = node.unwrap_or(self.root.clone());
        let mut dir_total = 0;
        for directory in node.borrow().directories.iter() {
            dir_total += self.find_sizes(Some(directory.clone()), result);
        }
        let file_sum = node.borrow().files.iter().map(|f| f.size).sum::<u64>();
        result.push(file_sum + dir_total);
        file_sum + dir_total
    }
}

fn parse_to_tree(input: &[String]) -> DirectoryTree {
    let tree = DirectoryTree {
        root: Rc::new(RefCell::new(DirectoryNode {
            parent: None,
            name: "/",
            files: vec![],
            directories: vec![],
        })),
    };

    let mut current_node = tree.root.clone();
    for line in input.iter().skip(2) {
        if line.starts_with("$ cd") {
            let name = &line[5..];
            if name == ".." {
                let new_node = current_node.borrow().parent.as_ref().unwrap().clone();
                current_node = new_node;
            } else {
                let new_node = current_node
                    .borrow()
                    .directories
                    .iter()
                    .find(|n| n.borrow().name == name)
                    .unwrap()
                    .clone();
                current_node = new_node;
            }
        } else if line.starts_with("dir") {
            let dir_name = &line[4..];
            current_node
                .borrow_mut()
                .directories
                .push(Rc::new(RefCell::new(DirectoryNode {
                    parent: Some(current_node.clone()),
                    name: dir_name,
                    files: vec![],
                    directories: vec![],
                })));
        } else if !line.starts_with("$ ls") {
            let (size, _) = line.split_once(' ').unwrap();
            current_node.borrow_mut().files.push(File {
                size: u64::from_str(size).unwrap(),
            });
        }
    }
    tree
}

pub fn part1(input: &[String]) -> u64 {
    let tree = parse_to_tree(input);
    let mut sizes = Vec::new();
    let _total_size = tree.find_sizes(None, &mut sizes);
    sizes.into_iter().filter(|&size| size <= 100000).sum::<u64>()
}

pub fn part2(input: &[String]) -> u64 {
    let tree = parse_to_tree(input);
    let mut sizes = Vec::new();
    let total_size = tree.find_sizes(None, &mut sizes);
    let unused_space = 70000000 - total_size;
    sizes.into_iter().filter(|&size| unused_space + size >= 30000000).min().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::{day7, parse_input_static};

    #[test]
    fn test_part1() {
        let input = parse_input_static("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k");

        assert_eq!(95437, day7::part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input_static("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k");

        assert_eq!(24933642, day7::part2(&input));
    }
}