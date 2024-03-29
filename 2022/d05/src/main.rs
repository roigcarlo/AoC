use std::io::{self, prelude::*};
use std::collections::HashMap;
use regex::Regex;
use std::time::Instant;

fn read_input() -> (HashMap<usize,Vec<String>>, Vec<Vec<usize>>) {
    let mut hm: HashMap<usize,Vec<String>> = HashMap::new();
    let mut mv: Vec<Vec<usize>> = Vec::new();
    
    let re_w = Regex::new(r"[A-Z]").unwrap();
    let re_d = Regex::new(r"\d+").unwrap();

    for line in io::stdin().lock().lines() {
        let ln = line.unwrap();
        if ln.contains("move") {
            mv.push(re_d.find_iter(ln.as_str()).map(|d| d.as_str().parse::<usize>().unwrap()).collect::<Vec<usize>>());
        } else if !ln.is_empty() {
            re_w.find_iter(ln.as_str()).for_each(|c| {hm.entry((c.start() / 4) + 1).and_modify(|v| v.push(String::from(c.as_str()))).or_insert(vec![String::from(c.as_str())]);} );
        }
    }

    for (_,v) in &mut hm {
        v.reverse();
    }

    (hm, mv)
}

fn stringify(hm: &HashMap<usize,Vec<String>>) -> String {
    (0..hm.len()).map(|x| hm.get(&(x+1)).expect("Invalid Key").last().unwrap().clone()).collect::<Vec<String>>().join("")
}

fn part1(hm: &HashMap<usize,Vec<String>>, mv: &Vec<Vec<usize>>) -> String {
    let mut hm = hm.clone();

    for movement in mv {
        if let [size, from, to] = &movement[..] {
            let src: &mut Vec<String> = hm.get_mut(from).expect("Invalid Key");
            let pack: Vec<String> = src.drain(src.len()-*size..).rev().collect();
            hm.entry(*to).and_modify(|x| x.extend(pack));
        }
    }

    stringify(&hm)
}

fn part2(hm: &HashMap<usize,Vec<String>>, mv: &Vec<Vec<usize>>) -> String {
    let mut hm = hm.clone();

    for movement in mv {
        if let [size, from, to] = &movement[..] {
            let src: &mut Vec<String> = hm.get_mut(from).expect("Invalid Key");
            let pack: Vec<String> = src.drain(src.len()-*size..).collect();
            hm.entry(*to).and_modify(|x| x.extend(pack));
        }
    }

    stringify(&hm)
}

fn main() -> io::Result<()> {
    let t_p0= Instant::now();
    let (hm, mv) = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1= Instant::now();
    let c9000 = part1(&hm, &mv);
    let e_p1 = t_p1.elapsed();

    let t_p2= Instant::now();
    let c9001 = part2(&hm, &mv);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] CrateMover 9000: {}\n", e_p1, c9000);

    print!("Part2 | ");
    print!("[{:.2?}] CrateMover 9001: {}\n", e_p2, c9001);

    Ok(())
}
