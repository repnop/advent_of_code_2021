pub fn run(input: &str) {
    let input = parse(input);
    println!("day 06 - part 1: {:?}", crate::rough_time(|| solution(&input, 80)));
    println!("day 06 - part 2: {:?}", crate::rough_time(|| solution(&input, 256)));
}

fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn solution(input: &[usize], days: usize) -> usize {
    let mut table = std::collections::VecDeque::from(vec![0; 9]);

    for input in input {
        table[*input] += 1;
    }

    for _ in 0..days {
        table.rotate_left(1);
        table[6] += table[8];
    }

    table.iter().copied().sum()
}
