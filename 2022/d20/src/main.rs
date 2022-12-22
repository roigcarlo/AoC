use std::io::{self, prelude::*};
use std::time::Instant;

fn read_input() -> Vec<i64> {
    let mut input: Vec<i64> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap().parse::<i64>().unwrap());
    }

    input
}

fn mixin(old_vec: &mut Vec<(usize, i64)>, input_len: usize) {
    for i in 0..input_len {
        let old_pos = old_vec.iter().position(|(k,_)| k==&i).unwrap();
        let new_pos = (old_pos as i64 + old_vec[old_pos].1).rem_euclid((input_len-1) as i64) as usize;

        let new_slc = [&old_vec[..old_pos],                      &old_vec[old_pos+1..]].concat();
        let new_vec = [&new_slc[..new_pos], &[old_vec[old_pos]], &new_slc[new_pos..  ]].concat();

        *old_vec = new_vec.to_vec();
    }
}

fn solve(input: &Vec<i64>, rep: usize, mul: i64) -> i64 {
    let input_len = input.len();
    let mut old_vec: Vec<(usize, i64)> = input.iter().map(|x| x * mul).enumerate().collect();

    for _ in 0..rep {
        mixin(&mut old_vec, input_len);
    }

    let zero = old_vec.iter().position(|(_,v)| v == &0).unwrap();

    old_vec[(zero+1000)%input_len].1 + 
    old_vec[(zero+2000)%input_len].1 + 
    old_vec[(zero+3000)%input_len].1
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let chnk = solve(&input, 1, 1);
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let skin = solve(&input, 10, 811589153);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Chnk: {}\n", e_p1, chnk);

    print!("Part2 | ");
    print!("[{:.2?}] Skin: {}\n", e_p2, skin);

    Ok(())
}
