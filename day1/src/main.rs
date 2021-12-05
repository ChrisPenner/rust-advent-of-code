use std::fs;

fn main() {
    part1();
    part2();
}
fn part1() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");

    let nums: Vec<u32> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse"))
        .collect();
    let sum: u32 = nums
        .windows(2)
        .map(|x| match x {
            [a, b] if a < b => 1,
            _ => 0,
        })
        .sum();

    println!("Part1: {}", sum);
}

fn part2() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");

    let nums: Vec<u32> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse"))
        .collect();
    let sum: u32 = nums
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|x| match x {
            [a, b] if a < b => 1,
            _ => 0,
        })
        .sum();

    println!("Part2: {}", sum);
}
