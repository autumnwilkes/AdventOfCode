use std::fs;

pub fn main() {
    let buff = fs::read_to_string("./input/day7.txt").unwrap();
    let parsed: Vec<(u64, Vec<u64>)> = buff
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(goal, values)| {
            (
                goal.parse::<u64>().unwrap(),
                values
                    .split(" ")
                    .map(|value| value.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .collect();
    let mut count = 0;
    concat(
        parsed.get(3).unwrap().0,
        parsed.get(3).unwrap().1.clone(),
        0,
        0,
    );
    for (target, values) in parsed {
        if mult(target, values.clone(), 1, 0)
            || add(target, values.clone(), 0, 0)
            || concat(target, values.clone(), 0, 0)
        {
            count += target;
        }
    }
    println!("{count}");
}

pub fn mult(target: u64, values: Vec<u64>, cur_sum: u64, index: usize) -> bool {
    if index == values.len() {
        return target == cur_sum;
    } else if target >= cur_sum * values.get(index).unwrap() {
        return mult(
            target,
            values.clone(),
            cur_sum * values.get(index).unwrap(),
            index + 1,
        ) || add(
            target,
            values.clone(),
            cur_sum * values.get(index).unwrap(),
            index + 1,
        ) || concat(
            target,
            values.clone(),
            cur_sum * values.get(index).unwrap(),
            index + 1,
        );
    } else {
        return false;
    }
}

pub fn add(target: u64, values: Vec<u64>, cur_sum: u64, index: usize) -> bool {
    if index == values.len() {
        return target == cur_sum;
    } else if target >= cur_sum + values.get(index).unwrap() {
        return mult(
            target,
            values.clone(),
            cur_sum + values.get(index).unwrap(),
            index + 1,
        ) || add(
            target,
            values.clone(),
            cur_sum + values.get(index).unwrap(),
            index + 1,
        ) || concat(
            target,
            values.clone(),
            cur_sum + values.get(index).unwrap(),
            index + 1,
        );
    } else {
        return false;
    }
}

pub fn num_length(value: u64) -> u64 {
    let mut count = 0;
    let mut power = 1;
    while value >= power {
        power *= 10;
        count += 1;
    }
    power
}

pub fn concat(target: u64, values: Vec<u64>, cur_sum: u64, index: usize) -> bool {
    if index == values.len() {
        return target == cur_sum;
    } else if target
        >= cur_sum * num_length(*values.get(index).unwrap()) + values.get(index).unwrap()
    {
        return mult(
            target,
            values.clone(),
            cur_sum * num_length(*values.get(index).unwrap()) + values.get(index).unwrap(),
            index + 1,
        ) || add(
            target,
            values.clone(),
            cur_sum * num_length(*values.get(index).unwrap()) + values.get(index).unwrap(),
            index + 1,
        ) || concat(
            target,
            values.clone(),
            cur_sum * num_length(*values.get(index).unwrap()) + values.get(index).unwrap(),
            index + 1,
        );
    } else {
        return false;
    }
}
