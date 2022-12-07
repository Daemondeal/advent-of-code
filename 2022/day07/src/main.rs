use std::env;
use std::fs;

#[derive(Debug)]
struct File {
    #[allow(dead_code)]
    name: String,
    size: i64,
}

#[derive(Debug)]
struct Folder {
    name: String,
    files: Vec<File>,
    children: Vec<usize>,
    parent: usize,
}

#[derive(Debug)]
struct FileSystem {
    folders: Vec<Folder>,
    cwd: usize,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            folders: vec![Folder {
                name: "/".to_owned(),
                files: vec![],
                children: vec![],
                parent: 0, // This is parent of itself
            }],
            cwd: 0,
        }
    }

    fn get_folder_size(&self, folder: usize) -> i64 {
        let folder = &self.folders[folder];

        let file_size: i64 = folder.files.iter().map(|x| x.size).sum();

        let folder_size: i64 = folder
            .children
            .iter()
            .map(|x| self.get_folder_size(*x))
            .sum();

        file_size + folder_size
    }

    fn add_directory(&mut self, name: String) -> usize {
        let parent = self.cwd;
        self.folders.push(Folder {
            name,
            children: vec![],
            files: vec![],
            parent,
        });

        let id = self.folders.len() - 1;

        self.folders[parent].children.push(id);

        id
    }

    fn add_file(&mut self, name: String, size: usize) {
        self.folders[self.cwd].files.push(File {
            name,
            size: size as i64,
        });
    }

    fn change_directory(&mut self, dirname: &str) -> bool {
        for folder in &self.folders[self.cwd].children {
            if self.folders[*folder].name == dirname {
                self.cwd = *folder;
                return true;
            }
        }

        false
    }

    fn change_directory_parent(&mut self) {
        self.cwd = self.folders[self.cwd].parent;
    }

    fn change_directory_root(&mut self) {
        self.cwd = 0;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("USAGE: cargo run -- [input_file]");
        return;
    }

    let input = fs::read_to_string(&args[1]).unwrap();

    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn process_input(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();

    let mut lines_iter = input.split('\n').peekable();

    while let Some(line) = lines_iter.next() {
        let mut params = line.split(' ');
        assert!(params.next().unwrap() == "$");

        match params.next().unwrap() {
            "cd" => match params.next().unwrap() {
                "/" => fs.change_directory_root(),
                ".." => fs.change_directory_parent(),
                other => _ = fs.change_directory(other),
            },
            "ls" => {
                while let Some(line) = lines_iter.peek() {
                    if line.starts_with('$') {
                        break;
                    }

                    // End of File
                    if line.is_empty() {
                        lines_iter.next();
                        break;
                    }

                    let mut params = line.split(' ');
                    let size = params.next().unwrap();
                    let Some(name_ref) = params.next() else {
                        let parent_name = &fs.folders[fs.cwd].name;

                        panic!("No name for children of folder {parent_name}.");
                    };

                    let name = name_ref.to_string();

                    if size == "dir" {
                        fs.add_directory(name);
                    } else {
                        fs.add_file(name, size.parse().unwrap());
                    }

                    lines_iter.next();
                }
            }
            other => {
                println!("Unexpected command {other}.")
            }
        }
    }

    fs
}

fn solve_a(input: &str) -> i64 {
    let fs = process_input(input);

    fs.folders
        .iter()
        .enumerate()
        .map(|(i, _)| fs.get_folder_size(i))
        .filter(|s| s <= &100000)
        .sum()
}

fn solve_b(input: &str) -> i64 {
    let fs = process_input(input);

    let unused: i64 = 70000000 - fs.get_folder_size(0);
    let space_needed: i64 = 30000000 - unused;

    fs.folders
        .iter()
        .enumerate()
        .map(|(i, _)| fs.get_folder_size(i))
        .filter(|s| s >= &space_needed)
        .min()
        .unwrap()
}
