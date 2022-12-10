use std::io::{self, prelude::*};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::Instant;

fn read_input() -> Vec<String> {
    io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}

fn part1(reader: &Vec<String>) -> u32 {
    let mut priority = 0;
    for line in reader.iter() {
        let half = line.chars().count()/2;
        let (p1,p2): (HashSet<char>, HashSet<char>) = (
            HashSet::from_iter(line[..half].chars()),
            HashSet::from_iter(line[half..].chars())
        );
        let r: char = *p1.iter().filter(|x| p2.contains(*x)).next().unwrap();

        priority += match Some(r as u32) {
            Some(x) if x <= ('Z' as u32) => x - ('A' as u32) + 27,
            Some(x) if x <= ('z' as u32) => x - ('a' as u32) + 1,
            Some(_)                      => unreachable!(),
            None                         => panic!(),
        };
    }

    priority
}

fn part2(reader: &Vec<String>) -> u32 {
    let mut grouping = 0;

    for lines in reader.chunks(3) {  
        let (p1,p2,p3): (HashSet<char>, HashSet<char>, HashSet<char>) = (
            HashSet::from_iter(lines[0].chars()),
            HashSet::from_iter(lines[1].chars()),
            HashSet::from_iter(lines[2].chars())
        );

        let r: char = *p1.iter().filter(|x| p2.contains(*x) && p3.contains(*x)).next().unwrap();

        grouping += match Some(r as u32) {
            Some(x) if x <= ('Z' as u32) => x - ('A' as u32) + 27,
            Some(x) if x <= ('z' as u32) => x - ('a' as u32) + 1,
            Some(_)                      => unreachable!(),
            None                         => panic!(),
        };   
    }

    grouping
}

fn main() -> io::Result<()> {
    let t_p0= Instant::now();
    let strategy = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1= Instant::now();
    let priority = part1(&strategy);
    let e_p1 = t_p1.elapsed();

    let t_p2= Instant::now();
    let grouping = part2(&strategy);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Priority: {}\n", e_p1, priority);

    print!("Part2 | ");
    print!("[{:.2?}] Grouping: {}\n", e_p2, grouping);

    Ok(())
}
