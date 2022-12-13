use std::io::{self, prelude::*};
use std::collections::VecDeque;
use std::time::Instant;

static NO_ORDER: u8 = 0;
static ORDER   : u8 = 1;
static FUZZY   : u8 = 2;

fn read_input() -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    
    for line in io::stdin().lock().lines() {
        input.push(line.unwrap());
    }

    input
}

fn chunk_deque(char_list: &mut VecDeque<char>) -> VecDeque<char> {
    let mut point = 0;
    let mut count = 0;
    
    while point < char_list.len() && char_list[point] != ',' || count != 0 {
        count += match char_list[point] {
            '[' =>  1,
            ']' => -1,
            _   =>  0,
        };
        point += 1;
    }

    if point == char_list.len() {
        char_list.drain(..point).collect::<VecDeque<char>>()
    } else {
        let mut front = char_list.drain(..point+1).collect::<VecDeque<char>>();
        front.pop_back();
        front
    }
}

fn cmp_list(tokens_l: &mut VecDeque<char>, tokens_r: &mut VecDeque<char>) -> u8 {
    tokens_l.pop_back(); tokens_l.pop_front();
    tokens_r.pop_back(); tokens_r.pop_front();

    let mut ordered = FUZZY;

    while !tokens_l.is_empty() && !tokens_r.is_empty() && ordered == FUZZY {
        let mut token_l = chunk_deque(tokens_l);
        let mut token_r = chunk_deque(tokens_r);

        // println!("{:?}, {:?}", token_l, token_r);

        match (token_l[0] != '[', token_r[0] != '[') {
            (true,true) => { 
                let l_val = Into::<Vec<char>>::into(token_l).iter().collect::<String>().parse::<u8>().unwrap();
                let r_val = Into::<Vec<char>>::into(token_r).iter().collect::<String>().parse::<u8>().unwrap();
                
                // println!("\n{}, {}", l_val, r_val);

                if l_val != r_val { ordered = (l_val < r_val) as u8; }
            },
            (true,false) => { 
                token_l.push_front('['); 
                token_l.push_back(']'); 
                ordered = cmp_list(&mut token_l, &mut token_r); 
            },
            (false,true) => { 
                token_r.push_front('['); 
                token_r.push_back(']'); 
                ordered = cmp_list(&mut token_l, &mut token_r);
            },
            (false,false) => { 
                ordered = cmp_list(&mut token_l, &mut token_r);    
            },
        };
    }

    return match ordered {
        x if x == FUZZY => match (tokens_l.is_empty(), tokens_r.is_empty()) {
            (true,false) => ORDER,
            (false,true) => NO_ORDER,
            _            => FUZZY,
        }
        x => x,
    };
}

fn part1(packets: & Vec<String>) -> usize {
    let mut ordered = 0;

    for (index, pair) in packets.chunks(3).enumerate() {
        let mut p_l: VecDeque<char> = pair[0].chars().collect();
        let mut p_r: VecDeque<char> = pair[1].chars().collect();
        let eval = cmp_list(&mut p_l, &mut p_r);
        if  eval == ORDER { ordered += index + 1; }
    }

    ordered
}

fn part2(packets: & Vec<String>) -> usize {
    let packets = packets.iter().filter(|x| !x.eq(&"")).map(|x| format!("{}",*x)).collect::<Vec<String>>();

    let mut top = 1;
    let mut mid = 2;

    for p in packets.iter() {
        let mut p1: VecDeque<char> = p.clone().chars().collect();
        let mut p2: VecDeque<char> = p.clone().chars().collect();
        if cmp_list(&mut p1, &mut "[[6]]".chars().collect::<VecDeque<char>>()) == 1 { mid += 1; }
        if cmp_list(&mut p2, &mut "[[2]]".chars().collect::<VecDeque<char>>()) == 1 { top += 1; }
    }

    top * mid
}

fn main() -> io::Result<()> {

    let t_p0= Instant::now();
    let input = read_input();
    let e_p0 = t_p0.elapsed();

    let t_p1 = Instant::now();
    let list = part1(&input);
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
