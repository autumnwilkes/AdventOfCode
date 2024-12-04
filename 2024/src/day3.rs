use std::fs;

pub fn main() {
    let buff = fs::read_to_string("./input/day3.txt").unwrap();
    let data: Vec<Vec<&str>> = buff
        .trim()
        .split("do()")
        .map(|x| x.split("don't()").collect::<Vec<&str>>()[0])
        .flat_map(|x| x.split("mul(").collect::<Vec<&str>>())
        .map(|x| x.split(",").collect::<Vec<&str>>())
        .collect();
    let a: Vec<u32> = data
        .iter()
        .filter(|x| x.len() != 1 && None != x.get(1).unwrap().find(")"))
        .map(|x| {
            [
                *x.get(0).unwrap(),
                &x.get(1).unwrap()[..x.get(1).unwrap().find(")").unwrap()],
            ]
        })
        .map(|x| {
            x.iter()
                .filter_map(|y| y.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .filter(|x| x.len() == 2)
        .map(|x| x.get(0).unwrap() * x.get(1).unwrap())
        .collect();
    println!("{:?}", a.iter().sum::<u32>());
}
