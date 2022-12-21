use std::io::{self, prelude::*};
use std::time::Instant;
use std::collections::HashMap;

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

fn do_step(tower: &mut Vec<u8>, seq: &Vec<char>, id_r: &mut usize, id_s: &mut usize, t_max: &mut usize) {
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

fn solve(input: &Vec<String>, limit: usize) -> usize {
    let seq = input[0].chars().collect::<Vec<char>>();
    
    let mut id_r = 0;  // Piece    IDX, 0..5
    let mut id_s = 0;  // Sequence IDX, 0..seq.len() 

    let mut tower: Vec<u8> = Vec::new();
    let mut t_max = 0;

    for _ in 0..5 {
        do_step(&mut tower, &seq, &mut id_r, &mut id_s, &mut t_max);
    }
    
    let mut perms: HashMap<(usize,Vec<u8>),(usize,usize)> = HashMap::new();

    for _ in 0..t_max {
        perms.insert((id_s,tower[0..t_max].to_vec()),(t_max,5));
    }

    let c_max = t_max;
    let mut cycled = false;
    let mut piece = 5;
    let mut at_end = (0,0);

    while !cycled {
        do_step(&mut tower, &seq, &mut id_r, &mut id_s, &mut t_max);
        piece += 1;
        
        if perms.contains_key(&(id_s,tower[t_max-c_max..t_max].to_vec())) { 
            cycled = true;
            at_end = (t_max,piece)
        } else {
            perms.insert((id_s,tower[t_max-c_max..t_max].to_vec()),(t_max,piece));
        }

    }

    let at_beg = perms.get(&(id_s,tower[t_max-c_max..t_max].to_vec())).unwrap();

    let ini_chunk = limit - at_beg.1;
    let cyc_chunk = ini_chunk / (at_end.1 - at_beg.1);

    let ini_size = at_beg.0;
    let cyc_size = cyc_chunk * (at_end.0 - at_beg.0);

    tower = tower[t_max-c_max..t_max].to_vec();

    let offs = tower.len();
    t_max = tower.len();

    for _ in at_beg.1 + cyc_chunk * (at_end.1 - at_beg.1) .. limit {
        do_step(&mut tower, &seq, &mut id_r, &mut id_s, &mut t_max);
    }

    ini_size + cyc_size + t_max - offs
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let smol = solve(&input, 2022usize);
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let long = solve(&input, 1000000000000usize);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Smol: {}\n", e_p1, smol);

    print!("Part2 | ");
    print!("[{:.2?}] Long: {}\n", e_p2, long);

    Ok(())
}
