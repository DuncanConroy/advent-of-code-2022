use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let stacks = restack_crates_and_return_top_crate(input.clone(), false);
    // println!("[Crate Mover 9000] The top crates are: {stacks:?}");
    //
    // let stacks = restack_crates_and_return_top_crate(input, true);
    // println!("[Crate Mover 9001] The top crates are: {stacks:?}");
}

fn parse_input(input: String) -> Vec<InputType> {
    input.lines().into_iter().map(InputType::from).collect()
}

fn build_file_system(input: String) -> FileSystem {
    let parsed_input = parse_input(input);
    let mut file_system = FileSystem::from(parsed_input);
    file_system
}

struct FileSystem {
    pub root: Node,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            root: Node::new(None, None),
        }
    }
}

impl From<Vec<InputType>> for FileSystem {
    fn from(input: Vec<InputType>) -> Self {
        let mut file_system = FileSystem::new();
        let mut dirs = vec![];
        let mut listing_directory = false;
        for (index, it) in input.iter().enumerate() {
            if index == 0 {
                let root = Node::new(None, Some(Directory { name: it.command.as_ref().unwrap().arguments.first().as_ref().unwrap().to_string() }));
                file_system = FileSystem { root };
                dirs.push(&file_system.root);
                continue;
            }

            if let Some(command) = &it.command {
                listing_directory = false;
                if command.name == "ls" {
                    listing_directory = true;
                } else if command.name == "cd" {
                    let dir = command.arguments.first().unwrap();
                    if dir != ".." {
                        dirs.push(&dirs.last_mut().unwrap().change_dir(command.arguments.first().unwrap()));
                    } else {
                        dirs.pop().unwrap();
                    }
                }
            } else if let Some(output) = &it.output {
                if !listing_directory {
                    eprintln!("Error: {output:?} is not a directory. Something is wrong here");
                }

                if let Some(file) = &output.file {
                    dirs.last_mut().unwrap().add_child(Rc::new(RefCell::new(Node::new(Some(file.clone()), None))));
                } else if let Some(directory) = &output.directory {
                    dirs.last_mut().unwrap().add_child(Rc::new(RefCell::new(Node::new(None, Some(directory.clone())))));
                }
            }
        }

        file_system
    }
}

#[derive(Debug)]
struct Node {
    pub file: Option<File>,
    pub directory: Option<Directory>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(file: Option<File>, directory: Option<Directory>) -> Self {
        Node { file, directory, children: vec![] }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }

    pub fn change_dir(&self, name: &str) -> &mut Node {
        self.children.iter().find(|it| it.borrow().directory.is_some() && it.borrow().directory.unwrap().name == name).unwrap().clone().get_mut()
    }
}

#[derive(Debug)]
struct InputType {
    pub command: Option<Command>,
    pub output: Option<Output>,
}

impl From<&str> for InputType {
    fn from(line: &str) -> Self {
        if line.starts_with("$") {
            InputType {
                command: Some(Command::from(line)),
                output: None,
            }
        } else {
            InputType {
                command: None,
                output: Some(Output::from(line)),
            }
        }
    }
}

#[derive(Debug)]
struct Command {
    pub name: String,
    pub arguments: Vec<String>,
}

impl From<&str> for Command {
    fn from(line: &str) -> Self {
        let mut parts = line.split(" ");
        parts.next(); // skip `$`
        let name = parts.next().unwrap().to_string();
        let arguments = parts.map(|it| it.to_string()).collect();
        Command {
            name,
            arguments,
        }
    }
}

#[derive(Debug)]
struct Output {
    directory: Option<Directory>,
    file: Option<File>,
}

impl From<&str> for Output {
    fn from(line: &str) -> Self {
        if line.starts_with("dir") {
            Output {
                directory: Some(Directory::from(line)),
                file: None,
            }
        } else {
            Output {
                directory: None,
                file: Some(File::from(line)),
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Directory {
    pub name: String,
}

impl From<&str> for Directory {
    fn from(line: &str) -> Self {
        let mut parts = line.split(" ");
        parts.next(); // skip `dir`
        let name = parts.next().unwrap().to_string();
        Directory {
            name
        }
    }
}

#[derive(Debug, Clone)]
struct File {
    pub name: String,
    pub size: usize,
}

impl From<&str> for File {
    fn from(line: &str) -> Self {
        let mut parts = line.split(" ");
        let size = parts.next().unwrap().to_string().parse::<usize>().unwrap();
        let name = parts.next().unwrap().to_string();
        File {
            name,
            size,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{build_file_system, parse_input};

    #[test]
    fn parses_command_and_outputs() {
        // given: lines of commands and outputs
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt"#.to_string();

        // when: parse is invoked
        let result = parse_input(input);

        // then: the output contains the correct objects
        assert_eq!(result.get(0).as_ref().unwrap().command.as_ref().unwrap().name, "cd");
        assert_eq!(result.get(0).as_ref().unwrap().command.as_ref().unwrap().arguments.get(0).unwrap(), "/");
        assert_eq!(result.get(1).as_ref().unwrap().command.as_ref().unwrap().name, "ls");
        assert_eq!(result.get(1).as_ref().unwrap().command.as_ref().unwrap().arguments.len(), 0);
        assert_eq!(result.get(2).as_ref().unwrap().output.as_ref().unwrap().directory.as_ref().unwrap().name, "a");
        assert_eq!(result.get(3).as_ref().unwrap().output.as_ref().unwrap().file.as_ref().unwrap().name, "b.txt");
        assert_eq!(result.get(3).as_ref().unwrap().output.as_ref().unwrap().file.as_ref().unwrap().size, 14848514);
    }

    #[test]
    fn builds_file_system_from_input() {
        // given: lines of commands and outputs
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt"#.to_string();

        // when: build_file_system is invoked
        let result = build_file_system(input);

        // then the file system is built correctly
    }
}
