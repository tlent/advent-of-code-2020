static INPUT: &str = include_str!("../input");

fn main() {
    let mut adapters: Vec<u32> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort();
    println!("part one: {}", part_one(&adapters));
    println!("part two: {}", part_two(&adapters));
}

fn part_one(adapters: &[u32]) -> u32 {
    let mut one_count = 0;
    let mut three_count = 1;
    let mut prev = 0;
    for &x in adapters {
        match x - prev {
            1 => one_count += 1,
            2 => {}
            3 => three_count += 1,
            _ => unreachable!(),
        }
        prev = x;
    }
    one_count * three_count
}

fn part_two(adapters: &[u32]) -> u64 {
    let mut paths = vec![0u64; adapters.len()];
    for (i, &v) in adapters[..3].iter().enumerate() {
        if v <= 3 {
            paths[i] = 1;
        }
    }
    for (i, &v) in adapters.iter().enumerate() {
        for j in 1..=3 {
            if i + j < adapters.len() && adapters[i + j] - v <= 3 {
                paths[i + j] += paths[i];
            }
        }
    }
    paths[paths.len() - 1]
}
