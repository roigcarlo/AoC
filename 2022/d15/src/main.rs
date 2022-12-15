use std::io::{self, prelude::*};
use std::time::Instant;

#[derive(Debug)]
struct RRange {
    range: Vec<(i32,i32)>,
    exlst: Vec<i32>,
}

impl RRange {
    fn add_range(&mut self, new_item: (i32,i32)) {
        
        let l = match self.range[0..].iter().enumerate().skip_while(|(_,x)| new_item.0 > x.1 ).map(|(i,_)| i).next() {
            Some(x) => x,
            None => usize::MAX,
        };

        let u = match self.range[0..].iter().enumerate().skip_while(|(_,x)| new_item.1 > x.1 ).map(|(i,_)| i).next() {
            Some(x) => x,
            None => usize::MAX,
        };

        match (l,u) {
            (usize::MAX,usize::MAX) => { 
                self.range.push(new_item); 
            }
            (x,usize::MAX) => {
                self.range[x].1 = new_item.1;
            }
            (usize::MAX, y) => {
                unreachable!("Eldritch condition.");
            }
            (x,y) if x == y => {
                self.range[x].0 = i32::min(new_item.0, self.range[x].0);
                self.range[x].1 = i32::max(new_item.1, self.range[x].1);
            }
            (x,y) if x!= y => {
                let mut t = y;
                
                let lb = i32::min(new_item.0, self.range[x].0);
                let mut ub = i32::max(new_item.1, self.range[x].1);

                if ub >= self.range[y].0 { 
                    t += 1;
                    ub = self.range[y].1;
                }
                
                let lhs = &self.range[..x];
                let rhs = &self.range[t..];

                let mut new_range: Vec<(i32,i32)> = Vec::new();
                new_range.extend_from_slice(lhs);
                new_range.push((lb, ub));
                new_range.extend_from_slice(rhs);

                self.range = new_range;
            }
            _ => {
                unreachable!("Eldritcher condition.");
            }
        }

        // println!("Inserted: {:?}", new_item);
        // println!("\t {:?}", self.range);
    }

    fn get_filled(&mut self) -> i32 {
        let mut filled = self.range.iter().map(|x| x.1 - x.0 + 1).sum();

        self.exlst.dedup();
        for bs in &self.exlst {
            filled -= self.range.iter().map(|x| if (x.0..=x.1).contains(&bs) {1} else {0}).sum::<i32>();
        }

        filled
    }
}

fn read_input() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap());
    }

    input
}

fn build_rows(input: &Vec<String>) -> Vec<Vec<i32>> {
    input.iter().map(|l| l.replace(&[',',':'],"")
        .split(" ").collect::<Vec<&str>>()
        .iter()
        .filter(|x| x.contains('='))
        .map(|x| x.split('=')
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|x| x[1]
                .parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .flatten()
        .collect()
    ).collect()
}

fn part1(input: &Vec<String>, at_row: i32) -> i32 {
    solve1(input, at_row, i32::MIN, i32::MAX).get_filled()
}

fn part2(input: &Vec<String>) -> u64 {
    let mut answer : u64 = 0;

    for i in 0..1 {
        let r = solve1(input, i, 0, 4000000);
        if r.range.len() == 2 {
            answer = (r.range[0].1 as u64 +1) * 4000000 + (i as u64);
            println!("{} {} {:?}", i, answer, r.range);
        }
    }

    // Too Slow
    answer
}

fn solve1(input: &Vec<String>, at_row: i32, minc: i32, maxc: i32) -> RRange {
    let mut ranges_at = RRange { range : Vec::new(), exlst: Vec::new() };
    let ranges_in = build_rows(&input);

    for coords in ranges_in {
        let sensor = (coords[0],coords[1]);
        let beacon = (coords[2],coords[3]);
    
        let ex_len = i32::abs(sensor.0 - beacon.0) + i32::abs(sensor.1 - beacon.1);

        let mut len = 0;
        let mut ori = 0;
        let mut end = 0;

        if at_row <= sensor.1 && at_row >= (sensor.1 - ex_len) {
            len = sensor.1 - at_row;
            ori = sensor.0 - i32::abs(ex_len - len);
            end = sensor.0 + i32::abs(ex_len - len);
            ranges_at.add_range((i32::max(i32::min(ori,maxc),minc),i32::max(i32::min(end,maxc),minc)));
        } 
        
        if at_row >= sensor.1 && at_row <= (sensor.1 + ex_len) {
            len = at_row - sensor.1;
            ori = sensor.0 - i32::abs(ex_len - len);
            end = sensor.0 + i32::abs(ex_len - len);
            ranges_at.add_range((i32::max(i32::min(ori,maxc),minc),i32::max(i32::min(end,maxc),minc)));
        }

        if sensor.1 == at_row {
            ranges_at.exlst.push(sensor.0);
        }

        if beacon.1 == at_row {
            ranges_at.exlst.push(beacon.0);
        }
    }

    ranges_at
}

fn solve2(input: &Vec<String>, at_row: i32, minc: i32, maxc: i32) -> RRange {
    let mut ranges_at = RRange { range : Vec::new(), exlst: Vec::new() };
    let ranges_in = build_rows(&input);

    for coords in ranges_in {
        let sensor = (coords[0],coords[1]);
        let beacon = (coords[2],coords[3]);
    
        let ex_len = i32::abs(sensor.0 - beacon.0) + i32::abs(sensor.1 - beacon.1);

        let mut len = 0;
        let mut ori = 0;
        let mut end = 0;

        if at_row <= sensor.1 && at_row >= (sensor.1 - ex_len) {
            len = sensor.1 - at_row;
            ori = sensor.0 - i32::abs(ex_len - len);
            end = sensor.0 + i32::abs(ex_len - len);
            ranges_at.add_range((i32::max(i32::min(ori,maxc),minc),i32::max(i32::min(end,maxc),minc)));
        } 
        
        if at_row >= sensor.1 && at_row <= (sensor.1 + ex_len) {
            len = at_row - sensor.1;
            ori = sensor.0 - i32::abs(ex_len - len);
            end = sensor.0 + i32::abs(ex_len - len);
            ranges_at.add_range((i32::max(i32::min(ori,maxc),minc),i32::max(i32::min(end,maxc),minc)));
        }

        if sensor.1 == at_row {
            ranges_at.exlst.push(sensor.0);
        }

        if beacon.1 == at_row {
            ranges_at.exlst.push(beacon.0);
        }
    }

    ranges_at
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let list = part1(&input, 2000000);
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let sort = part2(&input);
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] List: {}\n", e_p1, list);

    print!("Part2 | ");
    print!("[{:.2?}] Sort: {}\n", e_p2, sort);

    Ok(())
}
