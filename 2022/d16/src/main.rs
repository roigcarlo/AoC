use std::io::{self, prelude::*};
use std::collections::HashMap;
use std::time::Instant;

use itertools::Itertools;

type Cave = HashMap<String, Tunnel>;
type SpatialRift = HashMap<(String,String), usize>;

#[derive(Clone)]
struct Tunnel {
    valve: usize,
    exits: Vec<String>,
}

struct CaveSystem {
    c_cave: Cave,
    s_rift: SpatialRift,
    c_max: usize,
}

trait Problem {
    fn fill_spatial_rift(&mut self, at: &String, to: &String, d: &usize);
    fn build_spatial_rift(&mut self);
    fn build_cave_system(&mut self, input: &Vec<String>);

    fn solve(&mut self, left: &Vec<String>, at: &String, current: &usize, limit: &usize, pre: &usize) -> usize;
    fn part1(&mut self, input: &Vec<String>);
    fn part2(&mut self, input: &Vec<String>);
}

impl Problem for CaveSystem {
    fn fill_spatial_rift(&mut self, at: &String, to: &String, d: &usize) {
        let tc = self.c_cave.clone();
        for b in tc.get(to).unwrap().exits.iter() {
            let key = (at.to_string(),b.to_string());
            if !self.s_rift.contains_key(&key) || self.s_rift.get(&key) > Some(d) {
                self.s_rift.entry(key.clone()).and_modify(|x| *x = *d).or_insert(*d);
                self.fill_spatial_rift(at, &b, &(d+1));
            }
        }
    }
    
    fn build_spatial_rift(&mut self) {
        for (k,_) in self.c_cave.clone() { 
            self.s_rift.insert((k.to_string(),k.to_string()),0);
            self.fill_spatial_rift(&k.to_string(), &k.to_string(), &1);
        }
    }
    
    fn build_cave_system(&mut self, input: &Vec<String>) {
        self.c_cave = HashMap::new();
    
        for line in input {
            let valve_info = line.split(";").collect::<Vec<&str>>();
    
            let valve_id = &valve_info[0][6..8];
            let valve_tm = &valve_info[0][23..].parse::<usize>().unwrap();
            let cv_exits = &valve_info[1][23..].split(",").map(|x| x.trim().to_string()).collect::<Vec<String>>();
    
            self.c_cave.insert(
                valve_id.to_string(), Tunnel { 
                    valve: *valve_tm,
                    exits: cv_exits.to_vec()
                }
            );
        }
    }

    fn solve(&mut self, left: &Vec<String>, at: &String, current: &usize, limit: &usize, pre: &usize) -> usize {
    
        if left.is_empty() {
            let my_press = self.c_cave.get(at).unwrap().valve;
            let pre = pre + my_press * (limit-current);
            pre
        } else if current >= limit {
            *pre
        } else {
            left.iter().map(|to| {
                let mut new_left = left.clone();
                let my_press = self.c_cave.get(at).unwrap().valve;
                let index = new_left.iter().position(|x| x == to).unwrap();
                let dist_to_next = self.s_rift.get(&(at.to_string(),to.to_string())).unwrap() + 1;
                
                new_left.remove(index);
                let pre = pre + my_press * (limit-current);
                self.c_max = usize::max(self.c_max, pre);
                
                self.solve(&new_left, to, &(current + dist_to_next), limit, &pre)
            }).max().unwrap()
        }
    }
    
    fn part1(&mut self, input: &Vec<String>) {
        self.c_max = 0;
    
        self.build_cave_system(input);
        self.build_spatial_rift();
    
        let at = "AA";
    
        let left = self.c_cave.iter().filter(|(_,v)| v.valve != 0).map(|(k,_)| k.to_string()).collect::<Vec<String>>();
    
        let pressure = left.iter().map(|to| {
            let mut new_left = left.clone();
            let index = new_left.iter().position(|x| x == to).unwrap();
            new_left.remove(index);
            self.solve(&new_left, to, &(1+self.s_rift.get(&(at.to_string(),to.to_string())).unwrap()), &30, &0)
            
        }).max();
    
        self.c_max = pressure.unwrap()
    }
    
    fn part2(&mut self, input: &Vec<String>) {
        self.c_max = 0;
    
        self.build_cave_system(input);
        self.build_spatial_rift();
    
        let at = "AA";
    
        let left = self.c_cave.iter().filter(|(_,v)| v.valve != 0).map(|(k,_)| k.to_string()).collect::<Vec<String>>();
    
        let mut max_press = 0;
    
        for i in 1..=left.len()/2 {
            let perms = left.iter().combinations(i);
            println!("{:?}", perms);
            
            for left_perm in perms {
                let elf: Vec<String> = left_perm.iter().map(|s|s.clone().into()).collect();
                let ele: Vec<String> = left.iter().filter(|x| elf.iter().position(|y| y == *x).is_none()).map(|s|s.clone().into()).collect();
    
                let elf_p = elf.iter().map(|to| {
                    let mut new_left = elf.clone();
                    let index = new_left.iter().position(|x| x == to).unwrap();
                    new_left.remove(index);
                    self.solve(&new_left, to, &(1+self.s_rift.get(&(at.to_string(),to.to_string())).unwrap()), &26, &0)
                    
                }).max().unwrap();
    
                let ele_p = ele.iter().map(|to| {
                    let mut new_left = ele.clone();
                    let index = new_left.iter().position(|x| x == to).unwrap();
                    new_left.remove(index);
                    self.solve(&new_left, to, &(1+self.s_rift.get(&(at.to_string(),to.to_string())).unwrap()), &26, &0)
                    
                }).max().unwrap();
    
                max_press = usize::max(max_press, elf_p + ele_p);
            }
        }
    
        self.c_max = max_press
    }

}

fn read_input() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap());
    }

    input
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let mut cave_sys = CaveSystem {
        c_cave: Cave::new(),
        s_rift: SpatialRift::new(),
        c_max: 0,
    };

    let t_p1 = Instant::now();
    cave_sys.part1(&input);
    let list = cave_sys.c_max;
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    // Too Slow
    // cave_sys.part2(&input);
    let sort = cave_sys.c_max;
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] List: {}\n", e_p1, list);

    print!("Part2 | ");
    print!("[{:.2?}] Sort: {}\n", e_p2, sort);

    Ok(())
}
