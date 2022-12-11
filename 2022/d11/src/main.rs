use std::io::{self, prelude::*};
use std::time::Instant;

struct Monete {
    items: Vec<u64>,
    op: String,
    div: u64,
    throw: [usize; 2], 
}

fn read_input() -> Vec<String> {
    io::stdin().lock().lines().map(|x| x.unwrap()).collect()
}

fn solve(input: &Vec<String>, rounds: u64, reducer: &dyn Fn(u64) -> u64) -> u64 {
    let mut m_playground: Vec<Monete> = Vec::new();

    // : - (
    for m in input.chunks(7) {
        m_playground.push(
            Monete {
                items   : m[1][18..].split(",").map(|x| x.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>(),
                op      : m[2][19..].to_string(),
                div     : m[3][21..].parse::<u64>().unwrap(),
                throw   : [
                    m[4][29..].parse::<usize>().unwrap(),
                    m[5][30..].parse::<usize>().unwrap()
                ],
            }
        );
    }

    let base = m_playground.iter().map(|x| x.div).fold(1, |x,y| x*y);

    let mut inspections: Vec<u64> = vec![0; m_playground.len()];

    // : - )
    for _ in 0..rounds {
        for i in 0..m_playground.len() {
            let op_str  = m_playground[i].op.clone();
            let op_args = op_str.split(" ").collect::<Vec<&str>>();
            let div     = m_playground[i].div;

            let op = match op_args[1] {
                "+" => |x:u64, y:u64| -> u64 { x + y },
                "*" => |x:u64, y:u64| -> u64 { x * y },
                &_  => panic!("Eldritch operator found"),
            };
            
            let throw = m_playground[i].throw;
            let nitem = m_playground[i].items.len();

            inspections[i] += nitem as u64;

            for idx in 0..nitem {
                let item = m_playground[i].items[idx];
                
                let mut wl = match [op_args[0], op_args[2]] {
                    ["old", "old"] => op(item, item),
                    ["old", y    ] => op(item, y.parse::<u64>().unwrap()),
                    [x    , "old"] => op(x.parse::<u64>().unwrap(), item),
                    [x    , y    ] => op(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()),
                };

                wl = reducer(wl);
                wl = wl % base;

                if wl % div == 0 {
                    m_playground[throw[0]].items.push(wl);
                } else {
                    m_playground[throw[1]].items.push(wl);
                };
            }

            m_playground[i].items.clear();
        }
    }

    inspections.sort_by(|x,y| y.cmp(x));
    inspections[..2].iter().fold(1,|x,y| x*y)
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let relaxed = solve(&input, 20, &|x:u64| -> u64 { x / 3});
    let e_p1 = t_p1.elapsed();

    let t_p2 = Instant::now();
    let hystery = solve(&input, 10000, &|x:u64| -> u64 { x });
    let e_p2 = t_p2.elapsed();

    print!("Part0 | ");
    print!("[{:.2?}] I/O\n", e_p0);

    print!("Part1 | ");
    print!("[{:.2?}] Relaxed: {}\n", e_p1, relaxed);

    print!("Part2 | ");
    print!("[{:.2?}] Hystery: {}\n", e_p2, hystery);

    Ok(())
}
