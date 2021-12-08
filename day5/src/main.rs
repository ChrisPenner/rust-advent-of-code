use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{hash_map::RandomState, HashMap},
    fs, iter,
};

fn main() {
    day1();
    day2();
}

struct Point {
    x: u32,
    y: u32,
}

fn day1() {
    let numbers = Regex::new(r"\d+").unwrap();
    let coords: HashMap<(u32, u32), u32, RandomState> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|ln| {
            let ns: Vec<u32> = numbers
                .find_iter(ln)
                .map(|n| n.as_str().parse().unwrap())
                .collect();
            match ns[..] {
                [x1, y1, x2, y2] => (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }),
                _ => unreachable!(),
            }
        })
        .flat_map(|(p1, p2)| {
            let temp: Box<dyn Iterator<Item = (u32, u32)>> = if p1.x == p2.x {
                Box::new((min(p1.y, p2.y)..max(p1.y, p2.y) + 1).map(move |y| (p1.x, y)))
            } else if p1.y == p2.y {
                Box::new((min(p1.x, p2.x)..max(p1.x, p2.x) + 1).map(move |x| (x, p1.y)))
            } else {
                Box::new(iter::empty())
            };
            return temp;
        })
        .fold(HashMap::new(), |mut acc, (x, y)| {
            let v: &mut u32 = acc.entry((x, y)).or_insert(0);
            *v += 1;
            return acc;
        });

    let count = coords.iter().filter(|(_, v)| **v > 1).count();
    println!("{}", count);
}

fn day2() {
    let numbers = Regex::new(r"\d+").unwrap();
    let coords: HashMap<(u32, u32), u32, RandomState> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|ln| {
            let ns: Vec<u32> = numbers
                .find_iter(ln)
                .map(|n| n.as_str().parse().unwrap())
                .collect();
            match ns[..] {
                [x1, y1, x2, y2] => (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }),
                _ => unreachable!(),
            }
        })
        .flat_map(|(p1, p2)| {
            if p1.x == p2.x {
                (min(p1.y, p2.y)..max(p1.y, p2.y) + 1)
                    .map(move |y| (p1.x, y))
                    .collect::<Vec<_>>()
            } else if p1.y == p2.y {
                (min(p1.x, p2.x)..max(p1.x, p2.x) + 1)
                    .map(move |x| (x, p1.y))
                    .collect::<Vec<_>>()
            } else {
                let xs: Vec<u32> = if p1.x < p2.x {
                    (p1.x..p2.x + 1).collect()
                } else {
                    (p2.x..p1.x + 1).rev().collect()
                };
                let ys: Vec<u32> = if p1.y < p2.y {
                    (p1.y..p2.y + 1).collect()
                } else {
                    (p2.y..p1.y + 1).rev().collect()
                };
                xs.iter()
                    .zip(ys)
                    .map(|(x, y)| (*x, y))
                    .collect::<Vec<(u32, u32)>>()
            }
        })
        .fold(HashMap::new(), |mut acc, (x, y)| {
            let v: &mut u32 = acc.entry((x, y)).or_insert(0);
            *v += 1;
            return acc;
        });

    let count = coords.iter().filter(|(_, v)| **v > 1).count();
    println!("{}", count);
}
