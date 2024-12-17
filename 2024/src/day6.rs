use std::{cmp, fs};
#[derive(Clone, PartialEq, Debug)]
pub struct GuardState {
    pub pos: (usize, usize),
    pub direction: usize,
}

pub fn main() {
    let buff = fs::read_to_string("./input/day6.txt").unwrap();
    let mut guard: GuardState = GuardState {
        pos: (0, 0),
        direction: 0,
    };
    let mut prev_states: Vec<GuardState> = Vec::new();
    let mut map: Vec<Vec<char>> = buff
        .trim()
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, cell)| match cell {
                    '#' => '#',
                    '.' => '.',
                    '^' => {
                        guard.pos = (x, y);
                        '.'
                    }
                    err => panic!("invalid character in input of {err}"),
                })
                .collect()
        })
        .collect();
    let init_guard = guard.clone();
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut on_map = true;
    while on_map {
        if !visited.contains(&move_guard(&guard)) && !prev_states.contains(&guard) {
            prev_states.push(guard.clone());
        }
        if !visited.contains(&guard.pos) {
            visited.push(guard.pos.clone());
        }
        let mut new_guard_pos = move_guard(&guard);
        if new_guard_pos.0 < map.len() && new_guard_pos.1 < map.get(0).unwrap().len() {
            if map
                .get(new_guard_pos.0)
                .unwrap()
                .get(new_guard_pos.1)
                .unwrap()
                == &'#'
            {
                guard.direction = (guard.direction + 1) % 4;
            } else {
                guard.pos = new_guard_pos;
            }
        } else {
            on_map = false;
        }
    }

    let mut count = 0;
    for state in prev_states {
        guard = state.clone();
        let mut new_wall: (usize, usize) = move_guard(&state);
        if check(&guard, new_wall.clone(), map.clone())
            && map.get(new_wall.0).unwrap().get(new_wall.1).unwrap() != &'#'
        {
            count += 1;
        }
    }
    println!("{count}");
}

pub fn move_guard(guard: &GuardState) -> (usize, usize) {
    match guard.direction {
        0 => (guard.pos.0 - 1, guard.pos.1),
        1 => (guard.pos.0, guard.pos.1 + 1),
        2 => (guard.pos.0 + 1, guard.pos.1),
        3 => (guard.pos.0, guard.pos.1 - 1),
        _ => (0, 0),
    }
}

pub fn check(mut init_guard: &GuardState, new: (usize, usize), map: Vec<Vec<char>>) -> bool {
    let mut prev_states: Vec<GuardState> = Vec::new();
    let mut on_map = true;
    let mut guard = init_guard.clone();
    if map.get(new.0) == None || map.get(0).unwrap().get(new.1) == None {
        return false;
    }
    println!("{:?}", new);
    while on_map {
        if prev_states.contains(&guard) {
            return true;
        } else {
            prev_states.push(guard.clone());
        }
        if (guard.pos.0 == 0 && guard.direction == 0) || (guard.pos.1 == 0 && guard.direction == 3)
        {
            on_map = false;
        } else {
            let mut new_guard_pos = move_guard(&guard);
            if new_guard_pos.0 < map.len() && new_guard_pos.1 < map.get(0).unwrap().len() {
                if map
                    .get(new_guard_pos.0)
                    .unwrap()
                    .get(new_guard_pos.1)
                    .unwrap()
                    == &'#'
                {
                    guard.direction = (guard.direction + 1) % 4;
                } else if new_guard_pos == new {
                    guard.direction = (guard.direction + 1) % 4;
                } else {
                    guard.pos = new_guard_pos;
                }
            } else {
                on_map = false;
            }
        }
    }
    return false;
}
