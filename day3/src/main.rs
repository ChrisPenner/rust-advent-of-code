use std::fs;

fn main() {
    day1();
    println!("");
    println!("day 2");
    day2();
}

fn day1() {
    let mut counts = vec![0; 12];
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    let num_lines: i32 = i32::try_from(input.lines().count()).expect("out of bounds");
    input
        .lines()
        .flat_map(|s| s.chars().enumerate())
        .for_each(|(i, c)| match c {
            '1' => counts[i] += 1,
            _ => (),
        });

    let count_bin: Vec<i32> = counts
        .iter()
        .map(|i| (*i > (num_lines / 2)) as i32)
        .collect();
    let flipped_bin: Vec<i32> = count_bin.iter().map(|i| (*i == 0) as i32).collect();
    let gamma = from_binary(&count_bin);
    let epsilon = from_binary(&flipped_bin);
    println!(
        "Gamma rate: {}, Epsilon rate: {}, Total: {}",
        gamma,
        epsilon,
        gamma * epsilon,
    );
}

fn day2() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");

    let parsed: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let o2 = whittle(&parsed, |a, b| match (*a, *b) {
        (_, 2) => *a == 1,
        _ if *a == *b => true,
        _ => false,
    });
    let co2 = whittle(&parsed, |a, b| match (*a, *b) {
        (_, 2) => *a == 0,
        _ if *a == (if *b == 0 { 1 } else { 0 }) => true,
        _ => false,
    });

    let o2_dec = from_binary(&o2);
    // let rendered_o2: String = o2.iter().map::<String, _>(|c| c.to_string()).collect();
    println!("O2 {}", o2_dec);
    let co2_dec = from_binary(&co2);
    // let rendered_co2: String = co2.iter().map::<String, _>(|c| c.to_string()).collect();
    println!("CO2 {}", co2_dec);

    println!("Result {}", o2_dec * co2_dec);
}

fn whittle<'stat, 'outer>(
    opts: &'outer Vec<Vec<i32>>,
    matcher: fn(&i32, &i32) -> bool,
) -> Vec<i32> {
    let mut filtered = opts.clone();
    for i in 0..12 {
        let match_this = most_common_of(&filtered);
        filtered = filtered
            .into_iter()
            .filter(|elem| matcher((*elem).get(i).expect("!"), match_this.get(i).expect("!")))
            .collect();
        if filtered.len() == 1 {
            return filtered.get(0).expect("Expected single value").clone();
        }
    }
    unreachable!()
}

fn most_common_of(input: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut count_ones = [0; 12];
    let mut count_zeros = [0; 12];
    input.iter().for_each(|vec| {
        vec.iter().enumerate().for_each(|(i, c)| match *c {
            0 => count_zeros[i] += 1,
            1 => count_ones[i] += 1,
            _ => (),
        })
    });
    let most_common: Vec<i32> = count_zeros
        .iter()
        .zip(count_ones.iter())
        .map(|(zeros, ones)| {
            if *zeros > *ones {
                0
            } else if *ones > *zeros {
                1
            } else {
                2
            }
        })
        .collect();
    return most_common;
}

fn from_binary(v: &Vec<i32>) -> i32 {
    let base: i32 = 2;
    return v
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| base.pow(i.try_into().expect("!")) * b)
        .sum();
}
