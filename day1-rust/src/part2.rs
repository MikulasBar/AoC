use std::{collections::HashMap, hash::Hash};


const INPUT: &str = include_str!("./input.txt");

fn update_map(key: i32, map: &mut HashMap<i32, i32>) {
    match map.get_mut(&key) {
        Some(val) => *val += 1,
        None => _ = map.insert(key, 1),
    }
}

pub fn run() {
    let mut first: HashMap<i32, i32> = HashMap::new();
    let mut second: HashMap<i32, i32> = HashMap::new();

    INPUT.lines()
        .map(|line| {
            line.split_once("   ").unwrap()
        })
        .map(|(x, y)| (
            x.parse::<i32>().unwrap(),
            y.parse::<i32>().unwrap(),
        ))
        .for_each(|(x, y)| {
            update_map(x, &mut first);
            update_map(y, &mut second);
        });

    let result: i32 = first.iter()
        .map(|(k, v)| {
            let v2 = second.get(k).unwrap_or(&0);
            k * v * v2
        })
        .sum();

    println!("{}", result)
}