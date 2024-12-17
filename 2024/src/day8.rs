use std::collections::{HashMap, HashSet};
use std::fs;

pub fn main() {
    let buff = fs::read_to_string("./input/day8.txt").unwrap();
    let mut map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let bounds = (
        buff.lines()
            .collect::<Vec<&str>>()
            .len()
            .try_into()
            .unwrap(),
        buff.lines()
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .len()
            .try_into()
            .unwrap(),
    );
    buff.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, char)| {
            if map.contains_key(&char) {
                map.get_mut(&char)
                    .unwrap()
                    .push((x.try_into().unwrap(), y.try_into().unwrap()));
            } else {
                map.insert(char, vec![(x.try_into().unwrap(), y.try_into().unwrap())]);
            }
        })
    });
    map.remove(&'.');
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for positions in map.values() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i != j {
                    // let first = positions.get(i).unwrap();
                    // let second = positions.get(j).unwrap();
                    // let new = (2 * first.0 - second.0, 2 * first.1 - second.1);
                    // if new.0 >= 0 && new.0 < bounds.0 && new.1 >= 0 && new.1 < bounds.1 {
                    //     antinodes.insert(new);
                    // }

                    let mut init = positions.get(i).unwrap().clone();
                    let offest = positions.get(j).unwrap();
                    let difference = (init.0 - offest.0, init.1 - offest.1);
                    while init.0 >= 0 && init.0 < bounds.0 && init.1 >= 0 && init.1 < bounds.1 {
                        antinodes.insert(init.clone());
                        init.0 += difference.0;
                        init.1 += difference.1;
                    }
                }
            }
        }
    }
    println!("{}", antinodes.len());
}
