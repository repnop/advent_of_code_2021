pub fn run(input: &str) {
    let (line_len, input) = parse(input);
    println!("day 03 - part 1: {:?}", crate::rough_time(|| part_1(line_len, &input)));
    println!("day 03 - part 2: {:?}", crate::rough_time(|| part_2(line_len, &input)));
}

fn parse(input: &str) -> (usize, Vec<u32>) {
    (
        input.lines().next().unwrap().trim().len(),
        input.lines().map(|line| u32::from_str_radix(line.trim(), 2).unwrap()).collect(),
    )
}

fn part_1(line_length: usize, input: &[u32]) -> u32 {
    let mut position_counts = vec![(0, 0); line_length];

    for &input in input {
        for bit in 0..line_length {
            let (zero_count, one_count) = &mut position_counts[bit];

            if input & (1 << bit) == (1 << bit) {
                *one_count += 1;
            } else {
                *zero_count += 1;
            }
        }
    }

    let gamma_rate = position_counts
        .into_iter()
        .rev()
        .map(|(a, b)| (b > a) as u32)
        .fold(0, |gamma_rate, bit| (gamma_rate << 1) | bit);
    let epsilon_rate = (!gamma_rate) & ((1 << line_length) - 1);

    gamma_rate * epsilon_rate
}

#[test]
fn part_1_test() {
    let input = "00100
                 11110
                 10110
                 10111
                 10101
                 01111
                 00111
                 11100
                 10000
                 11001
                 00010
                 01010";

    let (line_len, input) = parse(input);
    assert_eq!(part_1(line_len, &input), 198);
}

fn part_2(line_length: usize, input: &[u32]) -> u32 {
    let oxygen_generator_rating = bit_filter(line_length, input, |zeroes, ones, bit| {
        match ((ones > zeroes) as u32, zeroes == ones, bit) {
            (mcb, false, bit) if bit == mcb => true,
            (_, true, 1) => true,
            _ => false,
        }
    });
    let co2_scrubber_rating = bit_filter(line_length, input, |zeroes, ones, bit| {
        match ((zeroes > ones) as u32, zeroes == ones, bit) {
            (mcb, false, bit) if bit == mcb => true,
            (_, true, 0) => true,
            _ => false,
        }
    });

    oxygen_generator_rating * co2_scrubber_rating
}

fn bit_filter(line_length: usize, input: &[u32], f: fn(usize, usize, u32) -> bool) -> u32 {
    let mut input = input.to_owned();
    let mut bit_position = line_length - 1;
    while input.len() != 1 {
        let (mut zero_count, mut one_count) = (0, 0);

        for &input in &input {
            if input & (1 << bit_position) == (1 << bit_position) {
                one_count += 1;
            } else {
                zero_count += 1;
            }
        }

        input = input
            .into_iter()
            .filter(|n| f(zero_count, one_count, (n >> bit_position) & 1))
            .collect();

        bit_position = bit_position.wrapping_sub(1);
    }

    input[0]
}

#[test]
fn part_2_test() {
    let input = "00100
                 11110
                 10110
                 10111
                 10101
                 01111
                 00111
                 11100
                 10000
                 11001
                 00010
                 01010";

    let (line_len, input) = parse(input);
    assert_eq!(part_2(line_len, &input), 230);
}
