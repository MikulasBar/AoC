

const INPUT: &str = include_str!("./input.txt");

// const INPUT: &str = "2333133121414131402";

pub fn run() {
    let mut mem = parse_input(INPUT);
    defragment(&mut mem);
    // println!("{:?}", mem);
    let res = check_sum(&mem);
    println!("{:?}", res);
}

fn check_sum(mem: &Vec<Memory>) -> u64 {
    let mut res = 0;
    let mut index = 0;
    for m in mem {
        if let Empty(s) = m {
            index += s;
            continue;
        }

        let File(id, s) = m else {panic!()};

        res += (id * sum_range(*s, index)) as u64;
        index += s;
    }

    res
}

fn defragment(mem: &mut Vec<Memory>) {
    let mut i = mem.len()-1;
    while i > 0 {
        if let Empty(..) = mem[i] {i -= 1; continue}

        for j in 0..i {
            if let File(..) = mem[j] {continue}

            if mem[j].size() >= mem[i].size() {
                let diff = mem[j].size() - mem[i].size();
                mem[j] = mem[i];
                mem[i] = Empty(mem[i].size());
                mem.insert(j + 1, Empty(diff));
                i += 1;
                break;
            }
        }

        i -= 1;
    }
}

fn sum_range(range: usize, index: usize) -> usize {
    range * (range + 2 * index - 1) / 2
}

fn parse_input(input: &str) -> Vec<Memory> {
    let mut alt = true;
    let mut vec = vec![];
    let mut id = 0;

    for ch in input.chars() {
        let n = ch.to_digit(10).unwrap();
        if alt {
            vec.push(Memory::File(id, n as usize));
            id += 1;
        } else {
            vec.push(Memory::Empty(n as usize));
        }

        alt = !alt;
    }

    vec
}

use Memory::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Memory {
    File(usize, usize),
    Empty(usize),
}

impl Memory {
    pub fn size(&self) -> usize {
        match *self {
            File(_, s) => s,
            Empty(s) => s,
        }
    }
}