use std::{io, io::prelude::*};
use std::collections::HashMap;
use std::time::Instant;

fn read_input() -> Result<Vec<(i32, i32)>, String> {
    let npc_map = HashMap::from([("A",0), ("B",1), ("C",2)]);
    let plr_map = HashMap::from([("X",0), ("Y",1), ("Z",2)]);
    
    let mut strategy = Vec::new();

    for line in io::stdin().lock().lines() {
        let data = line.unwrap();
        let data: Vec<_> = data.split(" ").collect();

        match &data[..] {
            &[f, s, ..] => strategy.push((
                npc_map.get(f).unwrap().clone(), 
                plr_map.get(s).unwrap().clone()
            )),
            _           => panic!(),
        };
    }

    if strategy.is_empty() {
        Err("Error reading the input".to_string())
    } else {
        Ok(strategy)
    }
}

fn part1(strategy: &Vec<(i32, i32)>) -> Result<i32, String> {
    match Some(strategy.iter().map(|(a,b)| (((b - a).rem_euclid(3)) + 1).rem_euclid(3) * 3 + b + 1).sum::<i32>()) {
        Some(x) if x >= 0   => Ok(x), 
        Some(x) if x <  0   => Err("Invalid Score.".to_string()),
        _                   => panic!(),
    }
}

fn part2(strategy: &Vec<(i32, i32)>) -> Result<i32, String> {
    match Some(strategy.iter().map(|(a,b)| (b * 3) + (a+b-1).rem_euclid(3) + 1).sum::<i32>()) {
        Some(x) if x >= 0   => Ok(x), 
        Some(x) if x <  0   => Err("Invalid Score.".to_string()),
        _                   => panic!(),
    }
}

fn main() -> io::Result<()> {
    let t_p0= Instant::now();
    let strategy = read_input().unwrap();
    let e_p0 = t_p0.elapsed();

    let t_p1= Instant::now();
    let score = part1(&strategy);
    let e_p1 = t_p1.elapsed();

    let t_p2= Instant::now();
    let cheat = part2(&strategy);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Score: {}\n", e_p1, score.unwrap());

    print!("Part2 | ");
    print!("[{:.2?}] Cheat: {}\n", e_p2, cheat.unwrap());

    Ok(())
}
