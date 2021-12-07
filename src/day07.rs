pub fn run(input: &str) {
    let input1 = parse(input);
    let input2 = input1.clone();
    println!("day 07 - part 1: {:?}", crate::rough_time(|| part_1(input1)));
    println!("day 07 - part 2: {:?}", crate::rough_time(|| part_2(input2)));
}

fn parse(input: &str) -> Vec<isize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn part_1(mut input: Vec<isize>) -> usize {
    input.sort_unstable();

    let mut fuel = 0;
    let median = input[input.len() / 2];

    for position in input {
        fuel += (position - median).abs() as usize;
    }

    fuel
}

fn part_2(input: Vec<isize>) -> usize {
    let mut fuel = usize::MAX;
    let average = (input.iter().copied().sum::<isize>() as f32 / input.len() as f32).round() as isize;
    let range = (average - average / 4)..(average + average / 4);

    for average in range {
        let mut this_fuel = 0;
        for &position in &input {
            let distance = (position - average).abs();
            this_fuel += (distance * (distance + 1) / 2) as usize;
        }

        fuel = usize::min(fuel, this_fuel);
    }

    fuel
}

#[test]
fn part_2_test() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    let input = parse(input);
    assert_eq!(part_2(input), 168);
}
