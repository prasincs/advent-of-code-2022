use std::{collections::HashMap, hash::Hash};

use anyhow::{bail, Error};
use itertools::Itertools;




#[derive(Debug, Clone,Default,PartialEq)]
struct File {
    name: String,
    size: u64,
}

struct Directory {
    name: String,
    files: HashMap<String, File>,
    directories: HashMap<String, Directory>,
}

impl Directory {

    fn get_files_size (&self) -> u64 {
        self.files.values().map(|file| file.size).sum()
    }

    fn get_total_size (&self) -> u64 {

        let mut size = self.get_files_size();

        for directory in get_all_directories_in(self) {
            size += directory.get_files_size()
        }

        size
    }

}

struct FileSystem {
    directories: HashMap<String, Directory>,
}

// cribbed partly from https://github.com/mvduijn/aoc2022/blob/main/day7/src/main.rs

enum ParsedLine {
    Ls,
    Cd(String),
    File(File),
    Directory(Directory),
    Unknown,
}
// I don't understand what &'a means - I know it's something about lifetimes
fn get_current_directory <'a> (file_system: &'a mut FileSystem, pwd: &Vec<String>) -> &'a mut Directory {

    let mut curdir = file_system.directories.get_mut(pwd.first().unwrap()).unwrap();

    for directory in pwd.iter().skip(1) {
        curdir = curdir.directories.get_mut(directory).unwrap()
    }

    curdir
}

fn get_all_directories_in (directory: &Directory) -> Vec<&Directory> {

    let mut directories: Vec<&Directory> = vec!();

    for directory in directory.directories.values() {
        directories.push(directory);
        directories.append(&mut get_all_directories_in(directory))
    }

    directories
}

fn parse_line(line: &str) -> ParsedLine{
    println!("parsing {}", line);
    if  line.starts_with("$") { // it's a command
        let cmds: Vec<&str> = line[1..].split_ascii_whitespace().collect();
        println!("{:?}", cmds);
        match cmds[0]{
            "ls" => ParsedLine::Ls,
            "cd" => ParsedLine::Cd(cmds[1].to_string()),
            _ => ParsedLine::Unknown,
        }
    }else {
        if line.starts_with("dir"){
            ParsedLine::Directory(Directory{name: line[4..].to_string(), files: HashMap::new(), directories: HashMap::new()})
        }else {
            let (size, name) = line.split(" ").tuples().next().unwrap();
            let file = File { size: size.parse().unwrap(), name: name.to_string() };
            ParsedLine::File(file)
        }
    }
}

fn get_filesystem(lines: Vec<String>) -> Result<FileSystem, Error> {
    let mut file_system = FileSystem{directories: HashMap::new()};

    let mut pwd: Vec<String> = vec!(String::from("/"));
    let root: Directory = Directory{ name: String::from("/"), files: HashMap::new(), directories: HashMap::new()};
    file_system.directories.insert(root.name.to_string(), root);
    for line in lines.iter().skip(2){ // dont need to parse `cd / and ls`
        match parse_line(line){
            ParsedLine::Ls => {}, // nothing to do here
            ParsedLine::Cd(name) => {
                if name == ".."{
                    pwd = pwd[0..pwd.len()-1].to_vec()
                } else {
                    pwd.push(name)
                }
            }
            ParsedLine::File(file) => {
                get_current_directory(&mut file_system, &pwd).files.insert(file.name.to_string(), file);
            }
            ParsedLine::Directory(directory) => {
                get_current_directory(&mut file_system, &pwd).directories.insert(directory.name.to_string(), directory);
            }
            ParsedLine::Unknown => bail!("Unexpected line {}", line)
        }
    }
    Ok(file_system)
}

fn part_one (file_system: &FileSystem) -> u64 {
    let mut all_directories = vec!(&file_system.directories["/"]);
    all_directories.append(&mut get_all_directories_in(&all_directories[0]));
    all_directories
    .iter()
    .map(|directory| directory.get_total_size())
    .filter(|size| *size <= 100000 as u64)
    .sum()
}

fn part_two (file_system: &FileSystem) -> u64 {

    let mut all_directories = vec!(&file_system.directories["/"]);
    all_directories.append(&mut get_all_directories_in(&all_directories[0]));

    let unused_space = 70000000 - &file_system.directories["/"].get_total_size();

    all_directories.iter()
        .map(|directory| directory.get_total_size())
        .filter(|size| *size + unused_space >= 30000000)
        .min()
        .unwrap()

}

pub fn run(){
    use crate::lines_from_file;
    let lines = lines_from_file("./inputs/day-7");
    let file_system = get_filesystem(lines).ok().unwrap();
    println!("part 1: {}", part_one(&file_system));
    println!("part 2: {}", part_two(&file_system));
}