use std::io::{self, prelude::*};
use std::collections::HashSet;
use std::iter::FromIterator;

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
    let input = read_input();

    let header  = solve(&input,  4);
    let message = solve(&input, 14);

    print!("Part1 | ");
    print!("Header : {}\n", header);

    print!("Part2 | ");
    print!("Message: {}\n", message);

    Ok(())
}
