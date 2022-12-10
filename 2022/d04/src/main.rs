use std::io::{self, prelude::*};
use std::time::Instant;

fn read_input() -> Vec<String> {
    io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}

fn full_overlap(r_pair: &String) -> u32 {
    match Some(r_pair.split(&['-',',']).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>().as_slice()) {
        Some([a,b,c,d]) => ((a >= c && b <= d) | (c >= a && d <= b)) as u32,
        Some([..])      => 0,
        None            => panic!(),
    }
}

fn part_overlap(r_pair: &String) -> u32 {
    match Some(r_pair.split(&['-',',']).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>().as_slice()) {
        Some([a,b,c,d]) => ((c >= a && c <= b) | (a >= c && a <= d)) as u32,
        Some([..])      => 0,
        None            => panic!(),
    }
}

fn part1(reader: &Vec<String>) -> u32 {
    reader.iter().map(|l| full_overlap(l)).sum()
}

fn part2(reader: &Vec<String>) -> u32 {
    reader.iter().map(|l| part_overlap(l)).sum()
}

fn main() -> io::Result<()> {
    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1= Instant::now();
    let full_overlap = part1(&input);
    let e_p1 = t_p1.elapsed();

    let t_p2= Instant::now();
    let part_overlap = part2(&input);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Full Overlap: {}\n", e_p1, full_overlap);

    print!("Part2 | ");
    print!("[{:.2?}] Part Overlap: {}\n", e_p2, part_overlap);

    Ok(())
}
