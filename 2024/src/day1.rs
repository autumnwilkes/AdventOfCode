use std::collections::HashMap;
use std::fs;
pub fn main() {
    let input = fs::read_to_string("./input/day1.txt").unwrap();
    let input_iter = input.trim().split_whitespace();
    let mut list_1: Vec<u32> = input_iter
        .clone()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, x)| x.parse().unwrap())
        .collect();
    let mut list_2: Vec<u32> = input_iter
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, x)| x.parse().unwrap())
        .collect();
    // list_1.sort();
    // list_2.sort();
    // let sum: u32 = list_1.iter().zip(list_2).map(|(a, b)| a.abs_diff(b)).sum();
    // println!("{sum}");

    let mut count: HashMap<u32, (u32, u32)> = HashMap::new();
    for i in list_1 {
        match count.get(&i) {
            Some(&(a, b)) => count.insert(i, (a + 1, b)),
            None => count.insert(i, (1, 0)),
        };
    }
    for i in list_2 {
        match count.get(&i) {
            Some(&(a, b)) => count.insert(i, (a, b + 1)),
            None => count.insert(i, (0, 1)),
        };
    }
    let sum: u32 = count.clone().iter().map(|(a, (b, c))| a * b * c).sum();
    println!("{}", sum);
    let c: u32 = count.iter().map(|(_, (b, c))| b + c).sum();
    println!("{}", c);
}
