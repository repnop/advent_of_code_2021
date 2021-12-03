pub fn run(input: &str) {
    let input = parse(input);
    println!("day 02 - part 1: {:?}", crate::rough_time(|| part_1(&input)));
    println!("day 02 - part 2: {:?}", crate::rough_time(|| part_2(&input)));
}

#[derive(Clone, Copy)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|s| {
            let (command, value) = s.split_once(' ').unwrap();
            let value = value.parse().unwrap();
            match command {
                "forward" => Command::Forward(value),
                "down" => Command::Down(value),
                "up" => Command::Up(value),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_1(input: &[Command]) -> u32 {
    let (horizontal, depth) =
        input.iter().copied().fold((0, 0), |(horizontal, depth), command| match command {
            Command::Forward(value) => (horizontal + value, depth),
            Command::Down(value) => (horizontal, depth + value),
            Command::Up(value) => (horizontal, depth.wrapping_sub(value)),
        });

    horizontal * depth
}

fn part_2(input: &[Command]) -> u32 {
    let (_, horizontal, depth) =
        input.iter().copied().fold((0, 0, 0), |(aim, horizontal, depth), command| match command {
            Command::Forward(value) => (aim, horizontal + value, depth + aim * value),
            Command::Down(value) => (aim + value, horizontal, depth),
            Command::Up(value) => (aim.wrapping_sub(value), horizontal, depth),
        });

    horizontal * depth
}
