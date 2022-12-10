use std::io::{self, prelude::*};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::Instant;

fn read_input() -> String {
    io::stdin().lock().lines().next().expect("Unable to read lines").unwrap()
}

fn count_same(header: &[char]) -> usize {
    HashSet::<&char>::from_iter(header.iter()).len()
}

fn solve(input: &String, win_size: usize) -> usize {
    input.chars().collect::<Vec<char>>().windows(win_size).enumerate().skip_while(|(_,h)| win_size != count_same(h)).next().unwrap().0 + win_size
}

fn main() -> io::Result<()> {
    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1= Instant::now();
    let header  = solve(&input,  4);
    let e_p1 = t_p1.elapsed();

    let t_p2= Instant::now();
    let message = solve(&input, 14);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Header : {}\n", e_p1, header);

    print!("Part2 | ");
    print!("[{:.2?}] Message: {}\n", e_p2, message);

    Ok(())
}
