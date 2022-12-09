use std::io::{self, prelude::*};
use std::collections::HashSet;
use std::time::Instant;

//    _____                       
//    |  __ \                      
//    | |__) |___  _ __   ___  ___ 
//    |  _  // _ \| '_ \ / _ \/ __|
//    | | \ \ (_) | |_) |  __/\__ \
//    |_|  \_\___/| .__/ \___||___/
//                | |  MultiPhysics          
//                |_| 
struct Rope {
    r: Vec<(i32,i32)>
}

trait RopesMultiphysics {             
    fn dist_t(&self) -> bool;
    fn dist_r(&self, dx: i32, dy: i32) -> bool;
    fn move_r(&mut self, to: &str, visited: &mut HashSet<(i32,i32)>);
}

impl RopesMultiphysics for Rope {
    fn dist_t(&self) -> bool {
        (1..self.r.len())
            .map(|k| i32::max(i32::abs(self.r[k-1].0 - self.r[k].0), i32::abs(self.r[k-1].1 - self.r[k].1)) > 1)
            .fold(false, |a,b| a | b)
    }

    fn dist_r(&self, dx: i32, dy: i32) -> bool {
        i32::max(i32::abs(dx), i32::abs(dy)) > 1 
    }

    fn move_r(&mut self, to: &str, visited: &mut HashSet<(i32,i32)>) {
        // Move the head
        let (dir, times) = if let [dir, times] = to.split(" ").collect::<Vec<&str>>()[..] { (dir, times) } else { todo!() };
        match (dir.chars().next().unwrap(), times.parse::<i32>().unwrap()) {
            ('U', x) => { self.r[0].1 += x; }
            ('D', x) => { self.r[0].1 -= x; }
            ('R', x) => { self.r[0].0 += x; }
            ('L', x) => { self.r[0].0 -= x; }
            _ => panic!("Not defined non invalid insturction not incorrectly. Don't not ignore this :(.")
        }

        // Move the tail
        while self.dist_t() {
            for k in 1..self.r.len() {
                let (dx, dy) = (   self.r[k-1].0 - self.r[k].0,                  self.r[k-1].1 - self.r[k].1);
                let (ix, iy) = (if self.r[k-1].0 > self.r[k].0 {1} else {-1}, if self.r[k-1].1 > self.r[k].1 {1} else {-1} );
                
                if self.dist_r(dx, dy) {
                    match (self.r[k-1].0, self.r[k-1].1, self.r[k].0, self.r[k].1) {
                        (a,b,c,d) if a != c && b != d => { self.r[k].0 += ix; 
                                                           self.r[k].1 += iy; }
                        (a,b,c,d) if a != c && b == d => { self.r[k].0 += ix; }
                        (a,b,c,d) if a == c && b != d => { self.r[k].1 += iy; }
                        (a,b,c,d) if a == c && b == d => { /* Relax and drink a cup of cafe con leche in plaza real */ }
                        _ => unreachable!(""),
                    }
                }

                if k == self.r.len() - 1 {
                    visited.insert((self.r[k].0, self.r[k].1));
                }
            }
        }
    }
}

fn read_input() -> Vec<String> {
    io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}

fn solve(trail: &Vec<String>, r_length: usize) -> usize {
    let mut places = HashSet::from([(0,0)]);
    let mut ropep1 = Rope {r: vec![(0,0); r_length]};

    for action in trail {
        ropep1.move_r(&action, &mut places);
    }

    places.len()
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let short = solve(&input, 2);
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let loong = solve(&input, 10);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Short: {}\n", e_p1, short);

    print!("Part2 | ");
    print!("[{:.2?}] Loong: {}\n", e_p2, loong);

    Ok(())
}
