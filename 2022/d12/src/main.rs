use std::io::{self, prelude::*};
use std::time::Instant;

fn read_input() -> Vec<Vec<char>> {
    let mut forest: Vec<Vec<char>> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        forest.push(line.unwrap()
            .chars()
            .collect()
        );
    }

    forest
}

fn get_at(input: &Vec<Vec<char>>, at: char) -> (usize,usize,i32) {
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == at {
                return (i,j,0);
            }
        }
    }

    return (0,0,-1);
}

fn fill_cell(walk_map: &mut Vec<Vec<i32>>, idx: usize, idy: usize, idw: i32) -> bool {
    if (walk_map[idx][idy] == -1) || (idw + 1 < walk_map[idx][idy]) {
        walk_map[idx][idy] = idw + 1;
        return true;
    }

    return false;
}

fn solve(input: &Vec<Vec<char>>, src: (char, char), dst: (char, char), eval: &dyn Fn(i32) -> bool) -> i32 {
    let mut size_map : Vec<Vec<char>> = input.to_vec();
    let mut walk_map : Vec<Vec<i32>>  = vec![vec![-1; input[0].len()]; input.len()];

    let mut front : Vec<(usize, usize, i32)> = Vec::new();
    let mut land  : Vec<i32> = Vec::new();

    let ini = get_at(input, src.0);
    let end = get_at(input, dst.0);

    let sx = size_map.len();
    let sy = size_map[0].len();

    size_map[ini.0][ini.1] = src.1;
    size_map[end.0][end.1] = dst.1;

    walk_map[ini.0][ini.1] = 0;

    front.push(ini);

    while !front.is_empty() {
        let (idx,idy,idw) = front[0];

        if size_map[idx][idy] == dst.1 {
            land.push(idw);
        }

        if (idx - 1) < sx && eval(size_map[idx-1][idy] as i32 - size_map[idx][idy] as i32) {
            if fill_cell(&mut walk_map, idx-1,idy,idw) { front.push((idx-1,idy,idw+1)); }
        }

        if (idy - 1) < sy && eval(size_map[idx][idy-1] as i32 - size_map[idx][idy] as i32) {
            if fill_cell(&mut walk_map, idx,idy-1,idw) { front.push((idx,idy-1,idw+1)); }
        }

        if (idx + 1) < sx && eval(size_map[idx+1][idy] as i32 - size_map[idx][idy] as i32) {
            if fill_cell(&mut walk_map, idx+1,idy,idw) { front.push((idx+1,idy,idw+1)); }
        }

        if (idy + 1) < sy && eval(size_map[idx][idy+1] as i32 - size_map[idx][idy] as i32) {
            if fill_cell(&mut walk_map, idx,idy+1,idw) { front.push((idx,idy+1,idw+1)); }
        }

        front = front[1..].iter().map(|x| *x).collect();
    }

    land.sort();
    land[0]
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let gaze = solve(&input,('S','a'),('E','z'),&|x:i32|->bool { x <  2 });
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let fast = solve(&input,('E','z'),('S','a'),&|x:i32|->bool { x > -2 });
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Gaze: {}\n", e_p1, gaze + 1);

    print!("Part2 | ");
    print!("[{:.2?}] Fast: {}\n", e_p2, fast);

    Ok(())
}
