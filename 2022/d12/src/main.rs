use std::io::{self, prelude::*};
use std::collections::VecDeque;
use std::time::Instant;

static EXACT: bool = true;
static SHORT: bool = false;

fn read_input() -> Vec<Vec<char>> {
    let mut mounts: Vec<Vec<char>> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        mounts.push(line.unwrap().chars().collect());
    }

    mounts
}

fn get_at(input: &Vec<Vec<char>>, at: char) -> (usize,usize,i32) {
    input.iter().flatten().enumerate().skip_while(|(_,&x)| x != at).take(1).map(|(k,_)| (k/input[0].len(),k%input[0].len(),0)).next().unwrap()
}

fn fill_cell(walk_map: &mut Vec<Vec<i32>>, idx: usize, idy: usize, idw: i32) -> bool {
    let insert = (walk_map[idx][idy] == -1) || (idw + 1 < walk_map[idx][idy]);
    
    if insert {
        walk_map[idx][idy] = idw + 1;
    }

    insert
}

fn solve(size_map: &mut Vec<Vec<char>>, src: (char, char), dst: (char, char), exact: bool, eval: &dyn Fn(i32) -> bool) -> i32 {
    let sx = size_map.len();
    let sy = size_map[0].len();

    let mut walk_map : Vec<Vec<i32>> = vec![vec![-1; sy]; sx];

    let mut front : VecDeque<(usize, usize, i32)> = VecDeque::new();
    let mut land  : Vec<i32> = Vec::new();

    let ini = get_at(size_map, src.0);
    let end = get_at(size_map, dst.0);

    size_map[ini.0][ini.1] = src.1;
    size_map[end.0][end.1] = dst.1;

    walk_map[ini.0][ini.1] = 0;

    front.push_back(ini);

    while !front.is_empty() {
        let (idx,idy,idw) = front.pop_front().unwrap();

        if size_map[idx][idy] == dst.1 {
            land.push(idw);
        }

        if (idx - 1) < sx && eval(size_map[idx-1][idy] as i32 - size_map[idx][idy] as i32) {
            if fill_cell(&mut walk_map, idx-1,idy,idw) { front.push_back((idx-1,idy,idw+1)); }
        }

        if (idy - 1) < sy && eval(size_map[idx][idy-1] as i32 - size_map[idx][idy] as i32) {
            if fill_cell(&mut walk_map, idx,idy-1,idw) { front.push_back((idx,idy-1,idw+1)); }
        }

        if (idx + 1) < sx && eval(size_map[idx+1][idy] as i32 - size_map[idx][idy] as i32) {
            if fill_cell(&mut walk_map, idx+1,idy,idw) { front.push_back((idx+1,idy,idw+1)); }
        }

        if (idy + 1) < sy && eval(size_map[idx][idy+1] as i32 - size_map[idx][idy] as i32) {
            if fill_cell(&mut walk_map, idx,idy+1,idw) { front.push_back((idx,idy+1,idw+1)); }
        }
    }

    size_map[ini.0][ini.1] = src.0;
    size_map[end.0][end.1] = dst.0;

    if exact {
        walk_map[end.0][end.1]
    } else {
        land.sort();
        land[0]
    }
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let mut input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let gaze = solve(&mut input,('S','a'),('E','z'),EXACT,&|x:i32|->bool { x <  2 });
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let fast = solve(&mut input,('E','z'),('S','a'),SHORT,&|x:i32|->bool { x > -2 });
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Gaze: {}\n", e_p1, gaze);

    print!("Part2 | ");
    print!("[{:.2?}] Fast: {}\n", e_p2, fast);

    Ok(())
}
