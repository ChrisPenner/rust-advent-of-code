use std::{
    collections::{hash_map::RandomState, HashSet},
    fs,
};

use regex::Regex;

fn main() {
    // day1();
    day2();
}

#[derive(Debug)]
struct BingoBoard {
    rows: Vec<Vec<i32>>,
}

impl BingoBoard {
    fn score(&self, calls: &HashSet<i32>) -> i32 {
        return self
            .rows
            .iter()
            .flat_map(|row| row.iter())
            .filter(|n| !calls.contains(n))
            .sum();
    }
    fn check(&self, calls: &HashSet<i32>) -> bool {
        let row_check = self
            .rows
            .iter()
            .any(|row| row.iter().all(|n| calls.contains(n)));
        let col_check = (0..5).into_iter().any(|col| {
            (0..5)
                .into_iter()
                .all(|row| calls.contains(&self.rows[row][col]))
        });
        return row_check || col_check;
    }
}

fn day1() {
    let numbers_re = Regex::new(r"\d+").unwrap();
    let bingo_calls_vec = vec![
        94, 21, 58, 16, 4, 1, 44, 6, 17, 48, 20, 92, 55, 36, 40, 63, 62, 2, 47, 7, 46, 72, 85, 24,
        66, 49, 34, 56, 98, 41, 84, 23, 86, 64, 28, 90, 39, 97, 73, 81, 12, 69, 35, 26, 75, 8, 32,
        77, 52, 50, 5, 96, 14, 31, 70, 60, 29, 71, 9, 68, 19, 65, 99, 57, 54, 61, 33, 91, 27, 78,
        43, 95, 42, 3, 88, 51, 53, 30, 89, 87, 93, 74, 18, 15, 80, 38, 82, 79, 0, 22, 13, 67, 59,
        11, 83, 76, 10, 37, 25, 45,
    ];

    // let bingo_calls_vec = vec![
    //     7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
    //     26, 1,
    // ];

    let input = fs::read_to_string("input.txt").unwrap();
    let bingo_boards: Vec<BingoBoard> = input
        .split("\n\n")
        .map(|board| {
            let rows = board
                .split('\n')
                .filter(|row| row.trim() != "")
                .map(|row| {
                    // println!("\"{}\"", row);
                    let row_result: Vec<i32> = numbers_re
                        .find_iter(row)
                        .map(|n| n.as_str().parse().unwrap())
                        .collect();
                    return row_result;
                })
                .collect();
            return BingoBoard { rows };
        })
        .collect();

    let mut bingo_calls: HashSet<_, RandomState> = HashSet::new();
    for call in bingo_calls_vec {
        bingo_calls.insert(call);
        match bingo_boards.iter().find(|b| b.check(&bingo_calls)) {
            Some(winner) => {
                println!("{:?}", bingo_calls.iter().collect::<Vec<&i32>>());
                println!("{:?}", winner);
                println!("{}, {}", winner.score(&bingo_calls), call);
                println!("{}", winner.score(&bingo_calls) * call);
                break;
            }
            None => (),
        };
    }
}

fn day2() {
    let numbers_re = Regex::new(r"\d+").unwrap();
    let bingo_calls_vec = vec![
        94, 21, 58, 16, 4, 1, 44, 6, 17, 48, 20, 92, 55, 36, 40, 63, 62, 2, 47, 7, 46, 72, 85, 24,
        66, 49, 34, 56, 98, 41, 84, 23, 86, 64, 28, 90, 39, 97, 73, 81, 12, 69, 35, 26, 75, 8, 32,
        77, 52, 50, 5, 96, 14, 31, 70, 60, 29, 71, 9, 68, 19, 65, 99, 57, 54, 61, 33, 91, 27, 78,
        43, 95, 42, 3, 88, 51, 53, 30, 89, 87, 93, 74, 18, 15, 80, 38, 82, 79, 0, 22, 13, 67, 59,
        11, 83, 76, 10, 37, 25, 45,
    ];

    let input = fs::read_to_string("input.txt").unwrap();
    let mut bingo_boards: Vec<BingoBoard> = input
        .split("\n\n")
        .map(|board| {
            let rows = board
                .split('\n')
                .filter(|row| row.trim() != "")
                .map(|row| {
                    // println!("\"{}\"", row);
                    let row_result: Vec<i32> = numbers_re
                        .find_iter(row)
                        .map(|n| n.as_str().parse().unwrap())
                        .collect();
                    return row_result;
                })
                .collect();
            return BingoBoard { rows };
        })
        .collect();

    let mut bingo_calls: HashSet<_, RandomState> = HashSet::new();
    for call in bingo_calls_vec {
        bingo_calls.insert(call);

        if bingo_boards.len() == 1 && bingo_boards.get(0).unwrap().check(&bingo_calls) {
            println!(
                "{}",
                bingo_boards.get(0).unwrap().score(&bingo_calls) * call
            );
            break;
        }
        bingo_boards = bingo_boards
            .into_iter()
            .filter(|b| !b.check(&bingo_calls))
            .collect();
    }
}
