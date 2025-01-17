use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone)]
struct File {
    pub len: u64,
    pub val: u64,
    pub trailing_free: u64,
}

impl File {
    pub fn add_following(&mut self, len: u64) {
        self.trailing_free += len;
    }
    pub fn set_following(&mut self, len: u64) {
        self.trailing_free = len;
    }
    pub fn get_sum(&self, index: u64) -> u64 {
        index * self.len * self.val + (self.len * (self.len - 1)) / 2 * self.val
    }
}

pub fn main() {
    let buff = fs::read_to_string("./input/day9.txt").unwrap();
    let mut files: Vec<File> = buff
        .trim()
        .chars()
        .tuples()
        .enumerate()
        .map(|(index, (len, trailing_free))| File {
            len: len.to_digit(10).unwrap().into(),
            trailing_free: trailing_free.to_digit(10).unwrap().into(),
            val: index.try_into().unwrap(),
        })
        .collect();

    let mut count = files.len() - 1;
    let mut prev_val = files.get(files.len() - 1).unwrap().val + 1;

    while count > 0 {
        if files.get(count).unwrap().val != prev_val - 1 {
            count -= 1;
        } else {
            let mut i = files.get_mut(count).unwrap().clone();
            prev_val = i.val;
            for second_index in 0..count {
                if i.len <= files.get(second_index).unwrap().trailing_free {
                    files
                        .get_mut(count - 1)
                        .unwrap()
                        .add_following(i.len + i.trailing_free);
                    i.set_following(files.get(second_index).unwrap().trailing_free - i.len);
                    files.get_mut(second_index).unwrap().set_following(0);
                    files.remove(count);
                    files.insert(second_index + 1, i.clone());
                    break;
                }
            }
        }
    }

    let mut index = 0;
    let mut sum = 0;

    for a in files {
        sum += a.get_sum(index);
        index += a.len + a.trailing_free;
    }
    println!("{sum}");
}
