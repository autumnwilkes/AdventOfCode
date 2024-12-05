use std::collections::{HashMap, HashSet};
use std::fs;

pub fn main() {
    let buff = fs::read_to_string("./input/day5.txt").unwrap();
    let mut data = buff.trim().split("\n\n");
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    data.next()
        .unwrap()
        .lines()
        .map(|x| x.split_once("|").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .for_each(|(first_page, second_page)| {
            match rules.get_mut(&first_page) {
                Some(set) => set.insert(second_page),
                None => rules
                    .insert(first_page, HashSet::from([second_page]))
                    .is_some(),
            };
        });
    let invalid_updates: Vec<Vec<u32>> = data
        .next()
        .unwrap()
        .split("\n")
        .map(|x| x.split(",").map(|y| y.parse().unwrap()).collect())
        .filter(|update| {
            let mut past_pages = HashSet::new();
            for page in update {
                if !past_pages.is_disjoint(&rules.get(page).unwrap_or(&HashSet::new())) {
                    return true;
                }
                past_pages.insert(page.clone());
            }
            return false;
        })
        .collect();
    let fixed_updates: Vec<Vec<u32>> = invalid_updates
        .iter()
        .map(|update| {
            let mut fixed: Vec<u32> = Vec::new();
            update.iter().for_each(|page| {
                for i in 0..fixed.len() {
                    if rules
                        .get(page)
                        .unwrap_or(&HashSet::new())
                        .contains(fixed.get(i).unwrap())
                    {
                        fixed.insert(i, *page);
                        break;
                    }
                }
                if !fixed.contains(page) {
                    fixed.push(*page);
                }
            });

            fixed
        })
        .collect();

    println!(
        "{}",
        fixed_updates
            .iter()
            .map(|x| x.get(x.len() / 2).unwrap())
            .sum::<u32>()
    )
}

// fn bad_update_fix(update: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
//     let mut correct_update = Vec::new();
//     for page in update {
//         for index in 0..correct_update.len() {
//             if rules
//                 .get(&page)
//                 .unwrap_or(&HashSet::new())
//                 .contains(correct_update.get(index).unwrap())
//             {
//                 correct_update.insert(index, *page);
//             }
//         }
//         if !correct_update.contains(&page) {
//             correct_update.push(*page)
//         }
//     }
//     correct_update
// }
