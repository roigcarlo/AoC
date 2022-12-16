use std::io::{self, prelude::*};
use std::collections::HashMap;
use std::time::Instant;

struct Tunnel {
    valve: usize,
    exits: Vec<String>,
}

type Cave = HashMap<String, Tunnel>;
type SynchroRift = HashMap<String, (Vec<u32>,Vec<String>)>;
type SpatialRift = HashMap<(String,String), usize>;

fn read_input() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap());
    }

    input
}

fn fill_spatial_rift(cave: &Cave, a: &String, b: &String, s_rift: &mut SpatialRift) -> usize {
    let src = std::cmp::min(a,b).to_string();
    let dst = std::cmp::max(a,b).to_string();

    let key = (src.to_string(), dst.to_string());

    // print!("key: {:?}",key);

    if a >= b {
        // println!(" Malformed Key");
        usize::MAX
    } else if a < b {
        // println!(" Is Contained? {}", s_rift.contains_key(&key));
        // let dist : usize = cave.get(b).unwrap().exits.iter().map(|x| fill_spatial_rift(cave,x,b,s_rift)).min().unwrap();
        if !s_rift.contains_key(&key) {
            // println!(" \t recursive call... {:?}", s_rift.keys());
            let dist = cave.get(b).unwrap().exits.iter().map(|x| fill_spatial_rift(cave,x,b,s_rift)).min().unwrap();
            s_rift.insert(key.clone(), 1+dist);
        }
        *s_rift.get(&key).unwrap()
    } else {
        usize::MAX   
    }
}

fn build_synchro_rift(cave: &Cave, period: usize) -> SynchroRift {
    let mut t_rift: SynchroRift = HashMap::new();

    t_rift
}

fn build_spatial_rift(cave: &Cave,) -> SpatialRift {
    let mut s_rift: SpatialRift = HashMap::new();

    for (i,v) in cave {
        for j in v.exits.iter() {
            let src = String::min(i.to_string(),j.to_string()).to_string();
            let dst = String::max(i.to_string(),j.to_string()).to_string();

            s_rift.entry((src,dst)).or_insert(1);
        } 
    }

    println!("s_rift: {:?}", s_rift);

    for (i,v) in cave {
        for j in cave.keys() {
            // println!("== {} = {} ==", i, j);
            fill_spatial_rift(cave, i, j, &mut s_rift);
        } 
    }

    println!("s_rift: {:?}", s_rift);

    for (k,v) in s_rift.clone() {
        s_rift.insert((k.1,k.0),v);
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

        println!("{:?}: {}: {:?}", valve_id, valve_tm, cv_exits);
    }

    cave_system
}

fn solve(cave: &Cave, left: &Vec<String>, s_rift: &SpatialRift, at: &String, current: &usize, limit: &usize, accum: &usize) -> usize {
    // println!("Choriquiso, choriso y quiso... {:?}", left);
    if left.is_empty() || current >= limit {
        *accum
    } else {
        left.iter().map(|to| {
            let mut new_left = left.clone();
            let my_press = cave.get(at).unwrap().valve;
            let new_accum = accum + my_press;
            let index = new_left.iter().position(|x| x == to).unwrap();
            let dist_to_next = s_rift.get(&(at.to_string(),to.to_string())).unwrap()+1;

            new_left.remove(index);
            // println!("\t sutamia {:?},{:?} <-- {:?} ({:?})", at, to, new_left,  &(s_rift.get(&(at.to_string(),to.to_string())))   );
        
            solve(&cave, &new_left, &s_rift, to, &(current + dist_to_next), limit, &new_accum)
        }).max().unwrap()
    }
}

fn part1(input: &Vec<String>) -> Option<u32> {
    let time_len = 30;

    let cave   = build_cave_system(input);
    let mut t_rift = build_synchro_rift(&cave, time_len);
    let mut s_rift = build_spatial_rift(&cave);

    let mut at = "AA";

    let left = cave.iter().filter(|(_,v)| v.valve != 0).map(|(k,_)| k.to_string()).collect::<Vec<String>>();

    let pressure = left.iter().map(|to| {
        let mut new_left = left.clone();
        let index = new_left.iter().position(|x| x == to).unwrap();
        new_left.remove(index);
        solve(&cave, &new_left, &s_rift, to, &(1+s_rift.get(&(at.to_string(),to.to_string())).unwrap()), &30, &0)
    }).max();

    println!("Ai que presion, que presion la vida... {:?}", pressure);

    // println!("String lexicon {}", String::max("AA".to_string(),"BB".to_string()));
    // println!("S-Rift: {:?}", s_rift);

    // for key in cave.keys() {
    //     rift.get_mut(&key)[time_len-1].0 = cave.get(&key).valve;
    //     rift.get_mut(&key)[time_len-1].1 = cave.get(&key).valve = cave.keys().to_vec();
    //     rift.get_mut(&key)[time_len-1].1.remove(key);
    // }

    // for i in (0..29).rev {
    //     if i % 2 == 0 {
    //         rift.get_mut(&key)[time_len] = cave.get(&key)[time_len+1];
    //     } else {
    //         rift.get_mut(&key)[time_len] = cave.get(&key)[time_len+1];
    //     }
    //     println!("Time {}:", i);
    // }

    Some(0)
}

fn part2(input: &Vec<String>) -> Option<u32> {
    Some(0)
}

fn main() -> io::Result<()> {

    // let t_p0= Instant::now();
    // let input = read_input();
    // let e_p0 = t_p0.elapsed();

    // let t_p1 = Instant::now();
    // let list = part1(&input);
    // let e_p1 = t_p1.elapsed();

    // let t_p2 = Instant::now();
    // let sort = part2(&input);
    // let e_p2 = t_p2.elapsed();

    // print!("Part0 | ");
    // print!("[{:.2?}] I/O\n", e_p0);

    // print!("Part1 | ");
    // print!("[{:.2?}] List: {}\n", e_p1, list.unwrap());

    // print!("Part2 | ");
    // print!("[{:.2?}] Sort: {}\n", e_p2, sort.unwrap());

    Ok(())
}
