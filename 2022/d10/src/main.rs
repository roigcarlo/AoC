use std::io::{self, prelude::*};
use std::time::Instant;

use d10::display_to_char;

fn read_input() -> Vec<String> {
    io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}

fn eval_cycle(cycle: &mut i32, grab: &mut i32, registry: &i32, signal: &mut i32) {
    if cycle > grab {
        *signal += *grab * registry;
        *grab += 40;
    }
}

fn draw(display: &mut Vec<char>, cycle: &i32, registry: &i32) {
    if ((cycle-1) % 40) > (registry-2) && ((cycle-1) % 40) < (registry+2) { display[(cycle-1) as usize] = '#'; }
}

fn part1(alu: &Vec<String>) -> (i32, Vec<char>) {
    let mut registry = 1;
    let mut cycle    = 1;
    let mut signal   = 0;
    let mut grab     = 20;

    let mut display  = vec![' '; 240];

    for l in alu {
        match &l.split(" ").collect::<Vec<&str>>()[..] {
            [inst, value] if inst.eq(&"addx") => {    
                cycle += 2;
                draw(&mut display, &(cycle-2), &registry);
                draw(&mut display, &(cycle-1), &registry);
                eval_cycle(&mut cycle, &mut grab, &registry, &mut signal);
                registry += value.parse::<i32>().unwrap();
            }
            [_, _] => panic!("Eldrich signal detected."),
            [inst] if inst.eq(&"noop") => { 
                cycle += 1;
                draw(&mut display, &(cycle-1)  , &registry);
                eval_cycle(&mut cycle, &mut grab, &registry, &mut signal);
            }
            [_,_,..] | [_] | [] => panic!("Eldrich signal detected."),
        }
    }

    (signal, display)
}

fn part2(display: &Vec<char>) -> String {
    let translator = |i: usize, s: &[char]| -> char { s[(i/6) + (i%6) * 40] };
    
    (0..240)
    .map(|i| translator(i,&display[..]))
    .collect::<Vec<char>>()
    .chunks(30)
    .map(|sl| display_to_char(&sl
        .iter()
        .take(24)
        .collect::<String>())
    ).collect::<String>()
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let (signal, display) = part1(&input);
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let sdisplay = part2(&display);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Signal : {}\n", e_p1, signal);

    print!("Part2 | ");
    print!("[{:.2?}] Display: {}\n", e_p1 + e_p2, sdisplay); // Ñiñiñi I will add both times because...

    Ok(())
}
