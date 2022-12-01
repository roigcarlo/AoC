use std::{io, io::prelude::*};

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
    let mut rustic_bread = read_input();
    let max_calories = part1(&rustic_bread);
    let top_calories = part2(&mut rustic_bread);

    print!("Part1 | ");

    match max_calories {
        Some(max_calories)  => print!("Max Calories: {}\n", max_calories),
        None                => print!("Fit and sleek, I eat no shit\n"),
    }

    print!("Part2 | ");

    print!("Top Calories: {}\n", top_calories);

    Ok(())
}
