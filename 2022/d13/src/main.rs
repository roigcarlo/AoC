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

fn chunk_deque(char_list: &mut VecDeque<char>) -> String {
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
        char_list.drain(..point).collect::<String>()[..point].to_string()
    } else {
        char_list.drain(..point+1).collect::<String>()[..point].to_string()
    }
}

fn cmp_list(p_l: String, p_r: String) -> u8 {
    let mut tokens_l: VecDeque<char> = p_l[1..p_l.len()-1].chars().collect();
    let mut tokens_r: VecDeque<char> = p_r[1..p_r.len()-1].chars().collect();

    let mut ordered = FUZZY;

    while !tokens_l.is_empty() && !tokens_r.is_empty() && ordered == FUZZY {
        let token_l = chunk_deque(&mut tokens_l);
        let token_r = chunk_deque(&mut tokens_r);

        match (token_l.parse::<u8>().is_ok(), token_r.parse::<u8>().is_ok()) {
            (true,true)   => { 
                let l_val = token_l.parse::<u8>().unwrap();
                let r_val = token_r.parse::<u8>().unwrap();

                if l_val != r_val { ordered = (l_val < r_val) as u8; }
            },
            (true,false)  => { ordered = cmp_list(format!("[{}]",token_l), token_r.to_string()) },
            (false,true)  => { ordered = cmp_list(token_l.to_string(), format!("[{}]",token_r)) },
            (false,false) => { ordered = cmp_list(token_l.to_string(), token_r.to_string())     },
        };
    }

    return match ordered {
        x if x == FUZZY => match (tokens_l.len(), tokens_r.len()) {
            (x,y) if x < y => ORDER,
            (x,y) if x > y => NO_ORDER,
            _              => FUZZY,
        }
        x     => x,
    };
}

fn part1(packets: & Vec<String>) -> usize {
    let mut ordered = 0;

    for (index, pair) in packets.chunks(3).enumerate() {
        let eval = cmp_list(pair[0].to_string(),pair[1].to_string());
        if  eval == ORDER { ordered += index + 1; }
    }

    ordered
}

fn part2(packets: & Vec<String>) -> usize {
    let mut packets = packets.iter().filter(|x| !x.eq(&"")).map(|x| format!("{}",*x)).collect::<Vec<String>>();
    
    packets.push("[[2]]".to_string());
    packets.push("[[6]]".to_string());

    packets.sort_by(|a,b| u8::cmp(&0,&cmp_list(a.to_string(),b.to_string())));

    let separator_1 = packets.iter().position(|x| x.eq(&"[[2]]".to_string())).unwrap() + 1;
    let separator_2 = packets.iter().position(|x| x.eq(&"[[6]]".to_string())).unwrap() + 1;

    separator_1 * separator_2
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
