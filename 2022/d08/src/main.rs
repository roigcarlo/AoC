use std::io::{self, prelude::*};
use std::time::Instant;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::cmp;

fn read_input() -> Vec<Vec<u32>> {
    let mut forest: Vec<Vec<u32>> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        forest.push(line.unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
        );
    }

    forest
}

trait AsMatrixExt<T> {
    fn row(&self, at: usize) -> Vec<&T>;
    fn col(&self, at: usize) -> Vec<&T>;
}

impl<T> AsMatrixExt<T> for Vec<Vec<T>> {
    fn row(&self, at: usize) -> Vec<&T>{
        self[at][..].iter()
            .collect::<Vec<&T>>()
    }

    fn col(&self, at: usize) -> Vec<&T> {
        self[..].iter()
            .map(|x| &x[at])
            .collect::<Vec<&T>>()
    }
}

fn get_view(row: usize, max: &u32, trees: &[&u32]) -> Vec<(usize,usize)> {
    let mut lm = max;
    trees[1..trees.len()-1].iter()
        .map(|n| {let m = lm; lm = cmp::max(lm, n); if n > &m { 1 } else { 0 } })
        .collect::<Vec<u32>>()
        .iter()
        .enumerate()
        .filter(|(_,v)| *v > &0)
        .map(|(k,_)| (row,k+1))
        .collect::<Vec<(usize,usize)>>()
}

// TODO: Change this shameless function
fn get_view_r(row: usize, max: &u32, trees: &[&u32]) -> Vec<(usize,usize)> {
    let mut lm = max;
    trees[1..trees.len()-1].iter().rev()
        .map(|n| {let m = lm; lm = cmp::max(lm, n); if n > &m { 1 } else { 0 } })
        .collect::<Vec<u32>>()
        .iter()
        .enumerate()
        .filter(|(_,v)| *v > &0)
        .map(|(k,_)| (row,trees.len()-k-2))
        .collect::<Vec<(usize,usize)>>()
}

fn part1(forest: &Vec<Vec<u32>>) -> usize {
    let l = forest.len();
    let mut views: HashSet<(usize,usize)> = HashSet::new();

    for i in 1..l-1 {
        for p in get_view(  i, &forest[i][0],   &forest.row(i)[..]) {
            views.insert(p);
        }
        for p in get_view(  i, &forest[0][i],   &forest.col(i)[..]).iter().map(|(x,y)| (*y,*x)) {
            views.insert(p);
        }
        for p in get_view_r(i, &forest[i][l-1], &forest.row(i)[..]) {
            views.insert(p);
        }
        for p in get_view_r(i, &forest[l-1][i], &forest.col(i)[..]).iter().map(|(x,y)| (*y,*x)) {
            views.insert(p);
        }
    }
    
    views.len() + (l * 4) - 4
}

fn part2(forest: &Vec<Vec<u32>>) -> usize {
    let f = forest.len();
    let mut m = 0;

    for i in 0..f {
        let row = forest.row(i);
        for j in 0..f {
            let col = forest.col(j);
            let tree = forest[i][j];

            let l = if j > 0   { match row[0  ..j].iter().rev().position(|&&x| x >= tree) { None => j,     x => x.unwrap() + 1 } } else { 0 };
            let r = if j < f-1 { match row[j+1..f].iter()      .position(|&&x| x >= tree) { None => f-j-1, x => x.unwrap() + 1 } } else { 0 };
            let u = if i > 0   { match col[0  ..i].iter().rev().position(|&&x| x >= tree) { None => i,     x => x.unwrap() + 1 } } else { 0 };
            let d = if i < f-1 { match col[i+1..f].iter()      .position(|&&x| x >= tree) { None => f-i-1, x => x.unwrap() + 1 } } else { 0 };

            m = cmp::max(m, l*r*u*d);
        }
    }
    
    m
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let forest = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let visible = part1(&forest);
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let special = part2(&forest);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Visible: {}\n", e_p1, visible);

    print!("Part2 | ");
    print!("[{:.2?}] Special: {}\n", e_p2, special);

    Ok(())
}
