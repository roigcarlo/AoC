use std::io::{self, prelude::*};
use std::time::Instant;

fn read_input() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap());
    }

    input
}

fn get_piece(id_r: usize) -> Vec<u8> {
    match id_r {
        0 => vec![0b0011110],
        1 => vec![0b0001000,
                  0b0011100,
                  0b0001000],
        2 => vec![0b0011100,
                  0b0000100,
                  0b0000100],
        3 => vec![0b0010000,
                  0b0010000,
                  0b0010000,
                  0b0010000],
        4 => vec![0b0011000,
                  0b0011000],
        _ => panic!("Eldritch piece"),
    }
}

fn extned_chunk(tower: &mut Vec<u8>, size: usize) {
    // Change this please
    for _ in 0..size {
        tower.push(0);
    }
}

fn pretty_print(tower: &Vec<u8>, at: usize, to: usize) {
    for i in (at..to).rev() {
        println!("|{:07b}|", tower[i]);
    }
    println!("+--------+");
}

fn part1(input: &Vec<String>) -> usize {
    let seq = input[0].chars().collect::<Vec<char>>();
    
    let mut id_r = 0;  // Piece    IDX, 0..5
    let mut id_s = 0;  // Sequence IDX, 0..seq.len() 

    let mut tower: Vec<u8> = Vec::new();
    let mut t_max = 0;

    for i in 0..2022 {
        let mut id_y = t_max + 3;
        let mut p = get_piece(id_r);

        if t_max + 3 + p.len() >= tower.len() {
            extned_chunk(&mut tower, 200);
        }

        let mut y_block = false;
        while !y_block {
            let mut x_block = false;

            // Check if movable sideways
            for j in 0..p.len() {
                match seq[id_s] {
                    '<' if !(p[j] < 0b1000000 && ((p[j] << 1) & tower[id_y+j]) == 0) => { x_block = true; } 
                    '>' if !(p[j] % 2 == 0    && ((p[j] >> 1) & tower[id_y+j]) == 0) => { x_block = true; } 
                    _   => {}
                }
            }

            // Move sideways
            if !x_block {
                for j in 0..p.len() {
                    p[j] = match seq[id_s] {
                        '<' => p[j] << 1,
                        '>' => p[j] >> 1,
                        _   => panic!("Eldritch operator")
                    }
                }
            }

            id_s = ( id_s + 1 ) % seq.len();

            // Check if bloqued
            y_block = id_y == 0 || (tower[id_y - 1] & p[0]) != 0;
            if id_r == 1 {
                y_block |= (tower[id_y] & p[1]) != 0;
            }

            // Fix
            if y_block {
                for j in 0..p.len() {
                    tower[id_y+j] = tower[id_y+j] | p[j];
                }
                t_max = usize::max(t_max, id_y + p.len());
            }

            // Move down
            id_y -= 1;
        }

        id_r = ( id_r + 1 ) % 5;
    }

    // pretty_print(&tower, 0, tower.len());

    t_max
}

fn do_cycle(tower: &mut Vec<u8>, seq: &Vec<char>, id_r: &mut usize, id_s: &mut usize, t_max: &mut usize) {
    let mut id_y = *t_max + 3;
    let mut p = get_piece(*id_r);

    if *t_max + 3 + p.len() >= tower.len() {
        extned_chunk(tower, 200);
    }

    let mut y_block = false;
    while !y_block {
        let mut x_block = false;

        // Check if movable sideways
        for j in 0..p.len() {
            match seq[*id_s] {
                '<' if !(p[j] < 0b1000000 && ((p[j] << 1) & tower[id_y+j]) == 0) => { x_block = true; } 
                '>' if !(p[j] % 2 == 0    && ((p[j] >> 1) & tower[id_y+j]) == 0) => { x_block = true; } 
                _   => {}
            }
        }

        // Move sideways
        if !x_block {
            for j in 0..p.len() {
                p[j] = match seq[*id_s] {
                    '<' => p[j] << 1,
                    '>' => p[j] >> 1,
                    _   => panic!("Eldritch operator")
                }
            }
        }

        *id_s = ( *id_s + 1 ) % seq.len();

        // Check if bloqued
        y_block = id_y == 0 || (tower[id_y - 1] & p[0]) != 0;
        if *id_r == 1 {
            y_block |= (tower[id_y] & p[1]) != 0;
        }

        // Fix
        if y_block {
            for j in 0..p.len() {
                tower[id_y+j] = tower[id_y+j] | p[j];
            }
            *t_max = usize::max(*t_max, id_y + p.len());
        }

        // Move down
        id_y -= 1;
    }

    *id_r = ( *id_r + 1 ) % 5;
}

fn part2(input: &Vec<String>) -> usize {
    let seq = input[0].chars().collect::<Vec<char>>();
    
    let mut id_r = 0;  // Piece    IDX, 0..5
    let mut id_s = 0;  // Sequence IDX, 0..seq.len() 

    let mut tower: Vec<u8> = Vec::new();
    let mut t_max = 0;

    for i in 0..5 {
        do_cycle(&mut tower, &seq, &mut id_r, &mut id_s, &mut t_max);
    }

    let mut initial: Vec<u8> = Vec::new();
    let mut c_max = t_max;

    for i in t_max-c_max..t_max {
        initial.push(tower[i]);
    }

    let mut cycled = false;
    let mut rounds = 0;

    let mut cycles: Vec<(usize, Vec<Vec<u8>>)> = Vec::new();

    cycles.push((id_s,vec![initial.clone()]));

    println!("Initial {:?}", initial);

    while !cycled {
        for i in 0..5 {
            do_cycle(&mut tower, &seq, &mut id_r, &mut id_s, &mut t_max);
        }

        rounds += 5;

        let mut local: Vec<u8> = Vec::new();

        for i in t_max-c_max..t_max {
            local.push(tower[i]);
        }

        for i in 0..cycles.len() {
            cycles[i].1.push(local.clone());
        }
        cycles.push((id_s,vec![local.clone()]));

        // println!("Local   {:?}", local);
        // if initial.iter().zip(&local).filter(|&(a, b)| a == b).count() == initial.len() {
        //     panic!("Found rep"); 
        // }

        // println!("{} {} {}", id_s == 0, id_r == 0, initial.iter().zip(&local).filter(|&(a, b)| a == b).count() == initial.len());
        // cycled = id_s == 0 && id_r == 0 && initial.iter().zip(&local).filter(|&(a, b)| a == b).count() == initial.len();
    }

    pretty_print(&tower, 0, 18);

    t_max
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let list = part1(&input);
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let sort = part1(&input);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] List: {}\n", e_p1, list);

    print!("Part2 | ");
    print!("[{:.2?}] Sort: {}\n", e_p2, sort);

    Ok(())
}
