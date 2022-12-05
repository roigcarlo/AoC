use std::io::{self, prelude::*};

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
    let input = read_input();

    let full_overlap = part1(&input);
    let part_overlap = part2(&input);

    print!("Part1 | ");
    print!("Full Overlap: {}\n", full_overlap);

    print!("Part2 | ");
    print!("Part Overlap: {}\n", part_overlap);

    Ok(())
}