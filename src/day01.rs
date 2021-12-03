use itertools::Itertools;

pub fn run(input: &str) {
    let input = parse(input);
    println!("day 01 - part 1: {:?}", crate::rough_time(|| part_1(&input)));
    println!("day 01 - part 2: {:?}", crate::rough_time(|| part_2(&input)));
}

fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.trim().parse().unwrap()).collect()
}

fn part_1(input: &[u32]) -> u32 {
    input.iter().copied().tuple_windows().map(|(a, b)| (b > a) as u32).sum()
}

fn part_2(input: &[u32]) -> u32 {
    input
        .iter()
        .copied()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(a, b)| (b > a) as u32)
        .sum()
}
