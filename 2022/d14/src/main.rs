use std::io::{self, prelude::*};
use std::time::Instant;
use std::collections::HashSet;
use std::collections::HashMap;

static SET : u8 = 0;
static USED: u8 = 1;
static FALL: u8 = 2;

fn read_input() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap());
    }

    input
}

fn cave_push(from: &(u32,u32), cave: &mut HashSet<(u32,u32)>, fall: &mut HashMap<u32,u32>, cave_min: i32, cave_max: i32, cave_floor: u32, on_floor: u8) -> u8 {
    if cave.contains(from) {
        return USED;
    }

    if from.1 > cave_floor {
        return on_floor;
    }

    if (from.0 as i32) < cave_min || (from.0 as i32) > cave_max {
        return FALL;
    }

    let pd = cave_push(&(from.0,from.1+1), cave, fall, cave_min, cave_max, cave_floor, on_floor);
    if pd == USED {
        let pl = cave_push(&(from.0-1,from.1+1), cave, fall, cave_min, cave_max, cave_floor, on_floor);
        if pl == USED {
            let pr = cave_push(&(from.0+1,from.1+1), cave, fall, cave_min, cave_max, cave_floor, on_floor);
            if pr == USED {
                cave.insert(*from);
                fall.entry(from.0).and_modify(|j| *j = u32::min(from.1,*j) ).or_insert(from.1);
                return SET;
            } else {
                return pr;
            }
        } else {
            return pl;
        }
    } else {
        return pd;
    }
}

fn build_cave(input: & Vec<String>) -> (HashSet<(u32,u32)>, HashMap< u32,u32 >) {
    let mut rocks: Vec<Vec<(u32,u32)>> = Vec::new();

    for rock in input {
        rocks.push(
            rock.split(" -> ").map(
                |x| x.split(',').map(
                    |y| y.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .chunks(2)
                .map(
                    |y| (y[0], y[1]))
                .collect::<Vec<(u32,u32)>>()
            )
            .flatten()
            .collect()
        );
    }

    let mut cave: HashSet<(u32,u32)> = HashSet::new();
    let mut fall: HashMap< u32,u32 > = HashMap::new();

    for rock in &rocks {
        for segment in rock.windows(2) {
            let src = segment[0];
            let dst = segment[1];

            match (src,dst) {
                ((x,y),(z,w)) if x == z => { 
                    let from = u32::min(y,w);
                    let to   = u32::max(y,w);
                    (from..=to).for_each(|i| { 
                        cave.insert((x,i)); 
                        fall.entry(x).and_modify(|j| *j = u32::min(i,*j) ).or_insert(i);
                    } ) }
                ((x,y),(z,w)) if y == w => { 
                    let from = u32::min(x,z);
                    let to   = u32::max(x,z);
                    (from..=to).for_each(|i| { 
                        cave.insert((i,y));
                        fall.entry(i).and_modify(|j| *j = u32::min(y,*j) ).or_insert(y);
                    } ) }
                _ => panic!("¯\\_(ツ)_/¯")
            }
        }
    }

    (cave, fall)
}

fn part1(input: & Vec<String>) -> usize {
    let (mut cave, mut fall) = build_cave(input);

    let cave_min   = fall.keys().min().unwrap().clone().try_into().unwrap();
    let cave_max   = fall.keys().max().unwrap().clone().try_into().unwrap();
    let cave_floor = cave.iter().map(|x| x.1).max().unwrap();

    let mut status = 0;
    let mut grains = 0;

    while status != FALL {
        let predict_h = fall.get(&500).unwrap();
        status  = cave_push(&(500,(*predict_h-1)), &mut cave, &mut fall, cave_min, cave_max, cave_floor, FALL);
        grains += 1;
    }

    grains - 1
}

fn part2(input: & Vec<String>) -> usize {
    let (mut cave, mut fall) = build_cave(input);

    let cave_min: i32 = i32::MIN;
    let cave_max: i32 = i32::MAX;
    let cave_floor = cave.iter().map(|x| x.1).max().unwrap() + 1;

    let mut grains = 0;

    while !cave.contains(&(500,0)) {
        let predict_h = fall.get(&500).unwrap();
        cave_push(&(500,*predict_h-1), &mut cave, &mut fall, cave_min, cave_max, cave_floor, USED);
        grains += 1;
    }

    grains
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
    print!("[{:.2?}] List: {}\n", e_p1, list);

    print!("Part2 | ");
    print!("[{:.2?}] Sort: {}\n", e_p2, sort);

    Ok(())
}
