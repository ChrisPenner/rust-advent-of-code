use std::fs;

fn main() {
    part1();
    part2();
}

struct Loc {
    depth: u32,
    position: u32,
}

fn part1() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");

    let mut loc = Loc {
        depth: 0,
        position: 0,
    };

    input
        .lines()
        .map(|s| match s.split(' ').collect::<Vec<&str>>()[..] {
            [cmd, n] => (cmd, n.parse::<u32>().expect("Bad number")),
            _ => panic!("Bad cmd"),
        })
        .for_each(|(cmd, n)| match cmd {
            "forward" => loc.position += n,
            "up" => loc.depth -= n,
            "down" => loc.depth += n,
            _ => panic!("Bad cmd"),
        });

    println!(
        "Depth: {}, Position: {}, total: {}",
        loc.depth,
        loc.position,
        loc.depth * loc.position
    )
}

fn part2() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");

    let mut loc = Loc {
        depth: 0,
        position: 0,
    };

    let mut aim = 0;

    input
        .lines()
        .map(|s| match s.split(' ').collect::<Vec<&str>>()[..] {
            [cmd, n] => (cmd, n.parse::<u32>().expect("Bad number")),
            _ => panic!("Bad cmd"),
        })
        .for_each(|(cmd, n)| match cmd {
            "forward" => {
                loc.position += n;
                loc.depth += aim * n
            }
            "up" => aim -= n,
            "down" => aim += n,
            _ => panic!("Bad cmd"),
        });

    println!(
        "Depth: {}, Position: {}, total: {}",
        loc.depth,
        loc.position,
        loc.depth * loc.position
    )
}
