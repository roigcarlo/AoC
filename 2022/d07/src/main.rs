use std::io::{self, prelude::*};
use std::collections::HashMap;
use std::error::Error;

struct INode {
    sub_nodes: HashMap<String, INode>,
    size: i32,
}

trait Node {
    fn insert_path(&mut self, path: &[String], desc: String);
    fn calc_size(&self) -> i32;
    fn pretty_print(&self, lvl: i32);
    fn get_subnodes(&self) -> Result<&HashMap<String, INode>, Box<dyn Error>>;
}

impl Node for INode {
    fn insert_path(&mut self, path: &[String], desc: String) {
        match path {
            [] => {
                let (meta, name) = if let Some((meta, name)) = desc.split_once(" ") { (meta, name) } else { todo!() };

                self.sub_nodes.insert(name.to_string(), INode {
                    sub_nodes: HashMap::new(),
                    size: if !meta.contains("dir") { meta.parse::<i32>().unwrap() } else { 0 },
                });
            }
            [current, sub @ ..] => {
                self.sub_nodes.get_mut(current).unwrap().insert_path(&sub[..], desc);
            }
        }
    }

    fn calc_size(&self) -> i32 {
        self.size + self.sub_nodes.iter().map(|(_,v)| v.calc_size()).sum::<i32>()
    }

    fn pretty_print(&self, lvl: i32) {
        for (k,v) in &self.sub_nodes {
            println!("{}{} - {}","\t".repeat(lvl as usize),k,v.calc_size());
            v.pretty_print(lvl+1);
        }
    }

    fn get_subnodes(&self) -> Result<&HashMap<String, INode>, Box<dyn Error>> {
        Ok(&self.sub_nodes)
    } 
}

fn read_input() -> Vec<String> {
    io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}

#[test]
fn check_tree_dir() -> Result<(), Box<dyn Error>> {
    let mut test_tree = INode {
        sub_nodes: HashMap::new(),
        size: 0,
    };

    let path: Vec<String> = Vec::new();

    test_tree.insert_path(&path[..], "300 File1.txt".to_string());
    test_tree.insert_path(&path[..], "500 File2.txt".to_string());

    assert_eq!(test_tree.calc_size(), 800);
    Ok(())
}

fn get_all_sizes(file_tree: &INode) -> Vec<i32> {
    let mut all_sizes: Vec<i32> = Vec::new();

    if file_tree.get_subnodes().unwrap().len() > 0 {
        all_sizes.push(file_tree.calc_size());
    }

    for (_,v) in file_tree.get_subnodes().unwrap().iter() {
        all_sizes.append(&mut get_all_sizes(v));
    }

    all_sizes
}

fn part1(file_tree: &INode, limit: &i32) -> i32 {
    get_all_sizes(file_tree)[1..].iter().filter(|&x| x <= limit).sum()
}

fn part2(file_tree: &INode, total: &i32, update: &i32) -> i32 {
    let size = &get_all_sizes(file_tree)[1..];
    let left = total - size[0];
    let need = update - left;

    *size.iter().filter(|&x| x - need >= 0).min().unwrap()
}

fn main() -> io::Result<()> {
    let input = read_input();

    let mut file_tree = INode {
        sub_nodes: HashMap::new(),
        size: 0,
    };

    file_tree.sub_nodes.insert("/".to_string(), INode { 
        sub_nodes : HashMap::new(),
        size: 0,
    });

    let mut path: Vec<String> = Vec::new();

    for line in input {
        if line.chars().next().unwrap().eq(&'$') {
            let command: Vec<&str> = line.split(" ").collect();
            match command[..] {
                [_, x, y] if x.eq("cd") && !y.eq("..")  => { path.push(y.to_string()); }
                [_, x, y] if x.eq("cd") &&  y.eq("..")  => { path.pop(); }
                [_, x]    if x.eq("ls")                 => { }
                [_, ..] | []                            => println!("This is not a command."),
            }
        } else {
            file_tree.insert_path(&path[..], line);
        }
    }

    let smoll_dirs = part1(&file_tree, &100000);
    let erase_dirs = part2(&file_tree, &70000000, &30000000);

    print!("Part1 | ");
    print!("Smoll Dirs: {}\n", smoll_dirs);

    print!("Part2 | ");
    print!("Erase Dirs: {}\n", erase_dirs);

    Ok(())
}
