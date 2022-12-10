use std::{io, io::prelude::*};
use std::time::Instant;

fn read_input() -> Vec<i32> {
    let mut rustic_bread = vec![0];
    let mut elf_num = 0;

    for line in io::stdin().lock().lines() {
        let data = line.unwrap();
        match data.as_str() {
            "" => {
                elf_num += 1;
                rustic_bread.push(0);
            },
            _  => rustic_bread[elf_num] += data.parse::<i32>().unwrap(),
        }
    }

    rustic_bread
}

fn part1(rustic_bread: &[i32]) -> Option<i32> {
    rustic_bread.iter().max().copied()
}

fn part2(rustic_bread: &mut Vec<i32>) -> i32 {
    rustic_bread.sort_by(|a, b| b.cmp(a));

    let mut top_calories = 0;

    for i in 0..3 {
        top_calories += rustic_bread[i];
    }

    top_calories 
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let mut rustic_bread = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1= Instant::now();
    let max_calories = part1(&rustic_bread).unwrap();
    let e_p1 = t_p1.elapsed();

    let t_p2= Instant::now();
    let top_calories = part2(&mut rustic_bread);
    let e_p2 = t_p2.elapsed();
    
    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Max Calories: {}\n", e_p1, max_calories);

    print!("Part2 | ");
    print!("[{:.2?}] Top Calories: {}\n", e_p2, top_calories);

    Ok(())
}
