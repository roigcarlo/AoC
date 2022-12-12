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
    if (walk_map[idx][idy] == -1) || (walk_map[idx][idy] > 0 && idw + 1 < walk_map[idx][idy]) {
        walk_map[idx][idy] = idw + 1;
        return true;
    }

    return false;
}

fn part2(input: &Vec<Vec<char>>) -> i32 {
    let mut min_path : i32 = i32::MAX;
    let mut size_map : Vec<Vec<char>> = input.to_vec();

    let ini = get_at(input, 'S');

    size_map[ini.0][ini.1] = 'a';

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'a' {
                size_map[i][j] = 'S';
                let local_min = part1(&size_map);
                if local_min > 0 {
                    min_path = i32::min(min_path, local_min);
                }
                size_map[i][j] = 'a';
            }
        }
    }

    min_path
}

fn part1(input: &Vec<Vec<char>>) -> i32 {
    let mut size_map : Vec<Vec<char>> = input.to_vec();
    let mut walk_map : Vec<Vec<i32>>  = vec![vec![-1; input[0].len()]; input.len()];

    let mut front : Vec<(usize, usize, i32)> = Vec::new();

    let ini = get_at(input, 'S');
    let end = get_at(input, 'E');

    size_map[ini.0][ini.1] = 'a';
    size_map[end.0][end.1] = 'z';

    walk_map[ini.0][ini.1] = 0;
    
    front.push(ini);

    while !front.is_empty() {
        let (idx,idy,idw) = front[0];

        if (idx - 1) < size_map.len() && (size_map[idx-1][idy] as i32 - size_map[idx][idy] as i32) < 2 {
            if fill_cell(&mut walk_map, idx-1,idy,idw) { front.push((idx-1,idy,idw+1)); }
        }

        if (idy - 1) < size_map[0].len() && (size_map[idx][idy-1] as i32 - size_map[idx][idy] as i32) < 2 {
            if fill_cell(&mut walk_map, idx,idy-1,idw) { front.push((idx,idy-1,idw+1)); }
        }

        if (idx + 1) < size_map.len() && (size_map[idx+1][idy] as i32 - size_map[idx][idy] as i32) < 2 {
            if fill_cell(&mut walk_map, idx+1,idy,idw) { front.push((idx+1,idy,idw+1)); }
        }

        if (idy + 1) < size_map[0].len() && (size_map[idx][idy+1] as i32 - size_map[idx][idy] as i32) < 2 {
            if fill_cell(&mut walk_map, idx,idy+1,idw) { front.push((idx,idy+1,idw+1)); }
        }

        front = front[1..].iter().map(|x| *x).collect();
    }

    walk_map[end.0][end.1]
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let gaze = part1(&input);
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let fast = part2(&input);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Gaze: {}\n", e_p1, gaze);

    print!("Part2 | ");
    print!("[{:.2?}] Fast: {}\n", e_p2, fast);

    Ok(())
}
