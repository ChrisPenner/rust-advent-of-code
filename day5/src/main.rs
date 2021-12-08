use regex::Regex;
use std::{
    collections::{hash_map::RandomState, HashMap},
    fs, iter,
};

fn main() {
    day1();
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
                .map(|n| dbg!(n.as_str()).parse().unwrap())
                .collect();
            match ns[..] {
                [x1, y1, x2, y2] => (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }),
                _ => unreachable!(),
            }
        })
        .flat_map(|(p1, p2)| {
            let temp: Box<dyn Iterator<Item = (u32, u32)>> = if p1.x == p2.x {
                Box::new((p1.y..p2.y).map(move |y| (p1.x, y)))
            } else if p1.y == p2.y {
                Box::new((p1.x..p2.x).map(move |x| (x, p1.y)))
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

    println!("{:?}", coords);
}
