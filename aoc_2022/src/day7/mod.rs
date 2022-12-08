use std::collections::HashMap;

// TODO: make this thing cleaner :)

#[derive(Debug)]
pub enum LsFile {
    File(u32, String),
    Dir(String),
}

#[derive(Debug)]
pub struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
pub struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<String>,
    parent: Option<String>,
}

#[derive(Debug)]
pub enum ElveCommand {
    Cd(String),
    Ls(Vec<LsFile>),
}

#[aoc_generator(day7)]
pub fn input_commands(input: &str) -> Vec<ElveCommand> {
    input
        .split("$").filter(|c| !c.is_empty()).map(|c| {
        let mut parts = c.lines().collect::<Vec<&str>>();
        parts.rotate_left(1);

        let command_with_params = parts.pop().unwrap().trim();
        let (command, params) = command_with_params.split_once(' ').unwrap_or((command_with_params, ""));

        match command {
            "ls" => {
                ElveCommand::Ls(parts.clone().into_iter().map(|s| {
                    let (size, name) = s.split_once(' ').unwrap();
                    if size == "dir" {
                        LsFile::Dir(name.to_string())
                    } else {
                        LsFile::File(size.parse::<u32>().unwrap(), name.to_string())
                    }
                }).collect::<Vec<LsFile>>())
            }
            "cd" => {
                ElveCommand::Cd(params.to_string())
            }
            _ => panic!("Unknown command: {}", command)
        }
    }).collect::<Vec<ElveCommand>>()
}

pub fn build_dirs(commands: &[ElveCommand]) -> HashMap::<String, Directory> {
    let mut dir_hash = HashMap::<String, Directory>::new();
    let root = Directory {
        name: "/".to_string(),
        files: vec![],
        directories: vec![],
        parent: None,
    };

    dir_hash.insert((&root.name).clone(), root);

    let mut current_dir = "/".to_string();

    for command in commands {
        match command {
            ElveCommand::Cd(dir) => {
                if dir == ".." {
                    current_dir = dir_hash.get(current_dir.as_str())
                        .as_ref().unwrap().parent.as_ref().unwrap().clone();
                    continue;
                }
                if dir == "/" {
                    current_dir = "/".to_string();
                    continue;
                }

                let new_dir_name = get_new_dir_name(current_dir.as_str(), dir.as_str());
                if dir_hash.contains_key(new_dir_name.as_str()) {
                    current_dir = new_dir_name;
                    continue;
                }

                let new_dir = Directory {
                    name: new_dir_name.clone(),
                    files: vec![],
                    directories: vec![],
                    parent: Some(current_dir.to_string()),
                };
                dir_hash.get_mut(current_dir.as_str()).unwrap().directories.push(new_dir_name.clone());
                current_dir = new_dir_name.clone();
                dir_hash.insert(new_dir_name, new_dir);
            }
            ElveCommand::Ls(files) => {
                for file in files {
                    match file {
                        LsFile::File(size, name) => {
                            if dir_hash.get(current_dir.as_str()).unwrap().files.iter().any(|f| f.name == *name) {
                                continue;
                            }
                            dir_hash.get_mut(current_dir.as_str()).unwrap().files.push(File {
                                name: name.to_string(),
                                size: *size,
                            });
                        }
                        LsFile::Dir(name) => {
                            let new_name = get_new_dir_name(current_dir.as_str(), name.as_str());
                            if dir_hash.contains_key(new_name.as_str()) {
                                continue;
                            }

                            dir_hash.insert(new_name.clone(), Directory {
                                name: new_name.clone(),
                                files: vec![],
                                directories: vec![],
                                parent: Some(current_dir.clone()),
                            });
                            dir_hash.get_mut(current_dir.as_str()).unwrap().directories.push(new_name.clone());
                        }
                    }
                }
            }
        };
    }

    dir_hash
}

pub fn get_sizes_flat(dir: String, directories: &HashMap<String, Directory>) -> Vec<u32> {
    let files_size: u32 = directories.get(&dir).unwrap().files.iter().fold(0, |acc, file| acc + file.size);
    let mut dir_sizes = directories.get(&dir).unwrap().directories
        .iter()
        .fold(vec![], |mut acc, dir| {
            acc.append(&mut get_sizes_flat(dir.to_string(), directories));
            acc
        });

    let current_dir_size = files_size + dir_sizes.iter().sum::<u32>();

    let mut all = vec![current_dir_size];
    all.append(&mut dir_sizes);
    all
}

pub fn space_used(dir: String, directories: &HashMap<String, Directory>) -> u32 {
    directories.get(&dir).unwrap().files.iter().fold(0, |acc, file| acc + file.size)
        + directories.get(&dir).unwrap().directories.iter().fold(0, |acc, dir| acc + space_used(dir.to_string(), directories))
}

pub fn get_dirs_that_free_enough(dir: String, directories: &HashMap<String, Directory>, needed: u32) -> Vec<u32> {
    let mut sizes = Vec::<u32>::new();

    let current_dir_size = space_used(dir.to_string(), directories);
    if current_dir_size >= needed {
        sizes.push(current_dir_size);
    }

    for dir in directories.get(&dir).unwrap().directories.iter() {
        sizes.append(&mut get_dirs_that_free_enough(dir.to_string(), directories, needed));
    }

    sizes
}

pub fn print_folder_structure(dir: String, directories: &HashMap<String, Directory>, current_tab: u32) -> () {
    if let Some(dir) = directories.get(&dir) {
        println!("{}- {} (dir)", "  ".repeat(current_tab as usize), dir.name);
        for file in dir.files.iter() {
            println!("{}- {} (file, size={})", "  ".repeat((current_tab + 1) as usize), file.name, file.size);
        }
        for dir in dir.directories.iter() {
            print_folder_structure(dir.to_string(), directories, current_tab + 1);
        }
    }
}

pub fn get_new_dir_name(parent: &str, new: &str) -> String {
    if parent == "/" {
        format!("{}", new)
    } else {
        format!("{}/{}", parent, new)
    }
}



#[aoc(day7, part1)]
pub fn part1(input: &[ElveCommand]) -> u32 {
    get_sizes_flat("/".to_string(), &build_dirs(input))
        .iter().filter(|s| **s < 100000).sum::<u32>()
}

#[aoc(day7, part2)]
pub fn part2(commands: &[ElveCommand]) -> u32 {
    let dirs = build_dirs(commands);
    let size_needed = 30000000 - (70000000 - space_used("/".to_string(), &dirs));
    get_dirs_that_free_enough("/".to_string(), &dirs, size_needed)
        .iter().min().unwrap().clone()
}