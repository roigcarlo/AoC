use std::io::{self, prelude::*};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::Instant;

fn read_input() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap());
    }

    input
}

fn get_chonks(input: &Vec<String>) -> Vec<(u16,u16,u16)> {
    input.iter()
        .map(|x| x.split(",").map(|y| y.parse::<u16>().unwrap()).collect())
        .map(|v: Vec<u16>| (v[0]+1, v[1]+1, v[2]+1))
        .collect()
}

fn get_obsidian(chonks: &Vec<(u16,u16,u16)>) -> HashMap<(u16,u16,u16), usize> {
    let mut obsidian: HashMap<(u16,u16,u16), usize> = HashMap::new();

    for c in chonks.iter() {
        let mut sides = 6;
        
        obsidian.entry((c.0-1,c.1,c.2)).and_modify(|x| {sides-=1; *x -= 1});
        obsidian.entry((c.0+1,c.1,c.2)).and_modify(|x| {sides-=1; *x -= 1});
        obsidian.entry((c.0,c.1-1,c.2)).and_modify(|x| {sides-=1; *x -= 1});
        obsidian.entry((c.0,c.1+1,c.2)).and_modify(|x| {sides-=1; *x -= 1});
        obsidian.entry((c.0,c.1,c.2-1)).and_modify(|x| {sides-=1; *x -= 1});
        obsidian.entry((c.0,c.1,c.2+1)).and_modify(|x| {sides-=1; *x -= 1});
    
        obsidian.insert(*c, sides);
    }

    obsidian
}

fn part1(input: &Vec<String>) -> usize {
    let chonks = get_chonks(input);

    let obsidian = get_obsidian(&chonks);

    obsidian.iter().map(|(_,v)| v).sum()
}

fn part2(input: &Vec<String>) -> usize {
    let chonks = get_chonks(input);

    let obsidian = get_obsidian(&chonks);
    let mut obsidian_skin: HashMap<(u16,u16,u16), usize> = HashMap::new();
    let mut air = HashSet::new();

    let mut front: VecDeque<(u16,u16,u16)> = VecDeque::new();
    front.push_back((0,0,0));
    air.insert((0,0,0));

    let max = chonks.iter().map(|(x,y,z)| u16::max(u16::max(*x,*y),*z)).max().unwrap() + 2;

    while front.len() > 0 {
        let c = front.pop_front().unwrap();

        if c.0-1 < max { if obsidian.contains_key(&(c.0-1,c.1,c.2)) { obsidian_skin.entry((c.0-1,c.1,c.2)).and_modify(|x| *x +=1 ).or_insert(1); } else { if !air.contains(&(c.0-1,c.1,c.2)) { air.insert((c.0-1,c.1,c.2)); front.push_back((c.0-1,c.1,c.2)); } } }
        if c.0+1 < max { if obsidian.contains_key(&(c.0+1,c.1,c.2)) { obsidian_skin.entry((c.0+1,c.1,c.2)).and_modify(|x| *x +=1 ).or_insert(1); } else { if !air.contains(&(c.0+1,c.1,c.2)) { air.insert((c.0+1,c.1,c.2)); front.push_back((c.0+1,c.1,c.2)); } } }
        if c.1-1 < max { if obsidian.contains_key(&(c.0,c.1-1,c.2)) { obsidian_skin.entry((c.0,c.1-1,c.2)).and_modify(|x| *x +=1 ).or_insert(1); } else { if !air.contains(&(c.0,c.1-1,c.2)) { air.insert((c.0,c.1-1,c.2)); front.push_back((c.0,c.1-1,c.2)); } } }
        if c.1+1 < max { if obsidian.contains_key(&(c.0,c.1+1,c.2)) { obsidian_skin.entry((c.0,c.1+1,c.2)).and_modify(|x| *x +=1 ).or_insert(1); } else { if !air.contains(&(c.0,c.1+1,c.2)) { air.insert((c.0,c.1+1,c.2)); front.push_back((c.0,c.1+1,c.2)); } } }
        if c.2-1 < max { if obsidian.contains_key(&(c.0,c.1,c.2-1)) { obsidian_skin.entry((c.0,c.1,c.2-1)).and_modify(|x| *x +=1 ).or_insert(1); } else { if !air.contains(&(c.0,c.1,c.2-1)) { air.insert((c.0,c.1,c.2-1)); front.push_back((c.0,c.1,c.2-1)); } } }
        if c.2+1 < max { if obsidian.contains_key(&(c.0,c.1,c.2+1)) { obsidian_skin.entry((c.0,c.1,c.2+1)).and_modify(|x| *x +=1 ).or_insert(1); } else { if !air.contains(&(c.0,c.1,c.2+1)) { air.insert((c.0,c.1,c.2+1)); front.push_back((c.0,c.1,c.2+1)); } } }
    }

    obsidian_skin.iter().map(|(_,v)| v).sum()
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let chnk = part1(&input);
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let skin = part2(&input);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Chnk: {}\n", e_p1, chnk);

    print!("Part2 | ");
    print!("[{:.2?}] Skin: {}\n", e_p2, skin);

    Ok(())
}
