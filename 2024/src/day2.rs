use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input/day2.txt").unwrap();
    let arr: i32 = input
        .trim()
        .split("\n")
        .map(|x| {
            is_safe_damp(
                x.split_whitespace()
                    .map(|y| y.parse::<i32>().unwrap())
                    .collect(),
            ) as i32
        })
        .sum();
    println!("{}", arr);
}

pub fn is_safe(report: Vec<i32>) -> i32 {
    let mut dec = report.get(0).unwrap() - report.get(1).unwrap();
    if dec == 0 {
        return 0;
    }
    dec = dec / dec.abs();
    let mut index = 0;
    let mut prev: i32 = report.get(0).unwrap() + dec;
    for level in report {
        if dec * level - dec * prev >= 0 || level.abs_diff(prev) > 3 {
            return index;
        }
        index += 1;
        prev = level;
    }

    return -1;
}

pub fn is_safe_damp(report: Vec<i32>) -> bool {
    let mut prev: i32 = 0;
    let mut index = 0;
    let mut incorrect_count: [i32; 2] = [0, 0];
    let mut incorrect_index: [i32; 2] = [-1, -1];
    for level in &report {
        if level - prev >= 0 || level.abs_diff(prev) > 3 {
            if index - incorrect_index[0] > 1 {
                if incorrect_count[0] > 1 {
                    incorrect_count[0] += 1;
                    break;
                }
                incorrect_count[0] += 1;
                incorrect_index[0] = index;
            }
        }
        prev = *level;
        index += 1;
    }
    index = 0;
    prev = 0;
    for level in &report {
        if level - prev <= 0 || level.abs_diff(prev) > 3 {
            if index - incorrect_index[1] > 1 {
                if incorrect_count[1] > 1 {
                    incorrect_count[1] += 1;
                    break;
                }
                incorrect_count[1] += 1;
                incorrect_index[1] = index;
            }
        }
        prev = *level;
        index += 1;
    }

    if incorrect_count[0] == 0 || incorrect_count[1] == 0 {
        return true;
    }
    if incorrect_count[0] == 1 {
        let mut new_report = report.clone();
        new_report.remove(incorrect_index[0] as usize);
        if is_safe(new_report) == -1 {
            return true;
        }
        new_report = report.clone();
        new_report.remove(incorrect_index[0] as usize - 1);
        if is_safe(new_report) == -1 {
            return true;
        }
    }
    if incorrect_count[1] == 1 {
        let mut new_report = report.clone();
        new_report.remove(incorrect_index[1] as usize);
        if is_safe(new_report) == -1 {
            return true;
        }
        new_report = report.clone();
        new_report.remove(incorrect_index[1] as usize - 1);
        if is_safe(new_report) == -1 {
            return true;
        }
    }
    return false;
}
