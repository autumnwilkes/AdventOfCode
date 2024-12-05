use std::collections::HashSet;
use std::fs;
pub fn main() {
    let buff = fs::read_to_string("./input/day4.txt").unwrap();
    let mut data: Vec<Vec<u8>> = buff
        .trim()
        .split_whitespace()
        .map(|x| x.as_bytes().to_vec())
        .collect();
    let mut sum: usize = 0;
    let key: Vec<Vec<u8>> = vec!["XMAS".as_bytes().to_vec(), "SAMX".as_bytes().to_vec()];
    /*    for row in 0..data.len() {
        for col in 0..data.get(0).unwrap().len() {
            if data.len() - row > 3 {
                if data.get(0).unwrap().len() - col > 3 {
                    if key.contains(&vec![
                        *data.get(row).unwrap().get(col).unwrap(),
                        *data.get(row + 1).unwrap().get(col + 1).unwrap(),
                        *data.get(row + 2).unwrap().get(col + 2).unwrap(),
                        *data.get(row + 3).unwrap().get(col + 3).unwrap(),
                    ]) {
                        sum += 1;
                        println!("row: {} col: {} direction: diag", row, col);
                    }
                }
                if key.contains(&vec![
                    *data.get(row).unwrap().get(col).unwrap(),
                    *data.get(row + 1).unwrap().get(col).unwrap(),
                    *data.get(row + 2).unwrap().get(col).unwrap(),
                    *data.get(row + 3).unwrap().get(col).unwrap(),
                ]) {
                    sum += 1;
                }
            }
            if data.get(0).unwrap().len() - col > 3 {
                if row > 2 {
                    if key.contains(&vec![
                        *data.get(row).unwrap().get(col).unwrap(),
                        *data.get(row - 1).unwrap().get(col + 1).unwrap(),
                        *data.get(row - 2).unwrap().get(col + 2).unwrap(),
                        *data.get(row - 3).unwrap().get(col + 3).unwrap(),
                    ]) {
                        sum += 1;
                    }
                }
                if key.contains(
                    &data.get(row).unwrap()[col..col + 4]
                        .iter()
                        .map(|x| *x)
                        .collect(),
                ) {
                    sum += 1;
                }
            }
        }
    } */

    let mas_letters = HashSet::from(["M".as_bytes()[0], "S".as_bytes()[0]]);
    for col in 1..data.len() - 1 {
        for row in 1..data.get(0).unwrap().len() - 1 {
            if data.get(col).unwrap().get(row).unwrap() == &"A".as_bytes()[0] {
                if mas_letters
                    == HashSet::from([
                        *data.get(col - 1).unwrap().get(row - 1).unwrap(),
                        *data.get(col + 1).unwrap().get(row + 1).unwrap(),
                    ])
                    && mas_letters
                        == HashSet::from([
                            *data.get(col - 1).unwrap().get(row + 1).unwrap(),
                            *data.get(col + 1).unwrap().get(row - 1).unwrap(),
                        ])
                {
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);
}
