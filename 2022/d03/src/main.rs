use std::io::{self, prelude::*};
use std::collections::HashSet;
use std::iter::FromIterator;
use itertools::Itertools;

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

    for lines in &reader.iter().chunks(3) {
        let group: Vec<&String> = lines.collect();
        
        let (p1,p2,p3): (HashSet<char>, HashSet<char>, HashSet<char>) = (
            HashSet::from_iter(group[0].chars()),
            HashSet::from_iter(group[1].chars()),
            HashSet::from_iter(group[2].chars())
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
    let input = read_input();

    let priority = part1(&input);
    let grouping = part2(&input);

    print!("Part1 | ");
    print!("Priority: {}\n", priority);

    print!("Part2 | ");
    print!("Grouping: {}\n", grouping);

    Ok(())
}
