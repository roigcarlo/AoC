use std::io::{self, prelude::*};
use std::collections::HashMap;
use std::time::Instant;

use itertools::Itertools;

struct Tunnel {
    valve: usize,
    exits: Vec<String>,
}

type Cave = HashMap<String, Tunnel>;
type SpatialRift = HashMap<(String,String), usize>;

fn read_input() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap());
    }

    input
}

fn get_pairs(tokens: &Vec<String>) -> Vec<(String, String)> {
    let mut res = Vec::new();

    for i in 0..tokens.len()-1 {
        for j in i+1..tokens.len() {
            res.push((tokens[i].clone(), tokens[j].clone()));
        }
    }

    res
}

fn fill_spatial_rift(cave: &Cave, at: &String, to: &String, d: &usize, s_rift: &mut SpatialRift) {
    for b in cave.get(to).unwrap().exits.iter() {
        let key = (at.to_string(),b.to_string());
        if !s_rift.contains_key(&key) || s_rift.get(&key) > Some(d) {
            *s_rift.entry(key.clone()).or_insert(*d) = *d;
            fill_spatial_rift(cave, at, &b, &(d+1), s_rift);
        }
    }
}

fn build_spatial_rift(cave: &Cave) -> SpatialRift {
    let mut s_rift: SpatialRift = HashMap::new();

    for k in cave.keys() { 
        s_rift.insert((k.to_string(),k.to_string()),0);
        fill_spatial_rift(cave, &k.to_string(), &k.to_string(), &1, &mut s_rift);
    }

    s_rift
}

fn build_cave_system(input: &Vec<String>) -> Cave {
    let mut cave_system: Cave = HashMap::new();

    for line in input {
        let valve_info = line.split(";").collect::<Vec<&str>>();

        let valve_id = &valve_info[0][6..8];
        let valve_tm = &valve_info[0][23..].parse::<usize>().unwrap();
        let cv_exits = &valve_info[1][23..].split(",").map(|x| x.trim().to_string()).collect::<Vec<String>>();

        cave_system.insert(
            valve_id.to_string(), Tunnel { 
                valve: *valve_tm,
                exits: cv_exits.to_vec()
            }
        );
    }

    cave_system
}

fn solve1(cave: &Cave, left: &Vec<String>, s_rift: &SpatialRift, at: &String, current: &usize, limit: &usize, pre: &usize) -> usize {
    
    if left.is_empty() {
        let my_press = cave.get(at).unwrap().valve;
        let pre = pre + my_press * (limit-current);
        pre
    } else if current >= limit {
        *pre
    } else {
        left.iter().map(|to| {
            let mut new_left = left.clone();
            let my_press = cave.get(at).unwrap().valve;
            let index = new_left.iter().position(|x| x == to).unwrap();
            let dist_to_next = s_rift.get(&(at.to_string(),to.to_string())).unwrap() + 1;
            
            new_left.remove(index);
            let pre = pre + my_press * (limit-current);

            solve1(&cave, &new_left, &s_rift, to, &(current + dist_to_next), limit, &pre)
        }).max().unwrap()
    }
}

fn part1(input: &Vec<String>) -> Option<usize> {
    let time_len = 30;

    let cave = build_cave_system(input);
    let mut s_rift = build_spatial_rift(&cave);

    let mut at = "AA";

    let left = cave.iter().filter(|(_,v)| v.valve != 0).map(|(k,_)| k.to_string()).collect::<Vec<String>>();

    let debug_vector : Vec<(String, usize)> = Vec::new();
    let pressure = left.iter().map(|to| {
        let mut new_left = left.clone();
        let index = new_left.iter().position(|x| x == to).unwrap();
        new_left.remove(index);
        solve1(&cave, &new_left, &s_rift, to, &(1+s_rift.get(&(at.to_string(),to.to_string())).unwrap()), &30, &0)
        
    }).max();

    pressure
}

fn part2(input: &Vec<String>) -> Option<usize> {
    let time_len = 30;

    let cave = build_cave_system(input);
    let mut s_rift = build_spatial_rift(&cave);

    let mut at = "AA";

    let left = cave.iter().filter(|(_,v)| v.valve != 0).map(|(k,_)| k.to_string()).collect::<Vec<String>>();

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
                solve1(&cave, &new_left, &s_rift, to, &(1+s_rift.get(&(at.to_string(),to.to_string())).unwrap()), &26, &0)
                
            }).max().unwrap();

            let ele_p = ele.iter().map(|to| {
                let mut new_left = ele.clone();
                let index = new_left.iter().position(|x| x == to).unwrap();
                new_left.remove(index);
                solve1(&cave, &new_left, &s_rift, to, &(1+s_rift.get(&(at.to_string(),to.to_string())).unwrap()), &26, &0)
                
            }).max().unwrap();

            max_press = usize::max(max_press, elf_p + ele_p);
        }
    }

    Some(max_press)
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let list = part1(&input);
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let sort = part2(&input);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] List: {}\n", e_p1, list.unwrap());

    print!("Part2 | ");
    print!("[{:.2?}] Sort: {}\n", e_p2, sort.unwrap());

    Ok(())
}
