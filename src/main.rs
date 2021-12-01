mod day01;

static DAYS: &'static [fn(&str)] = &[
    day01::run
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (i, day) in DAYS.iter().enumerate() {
        let input = std::fs::read_to_string(format!("input/day{:>02}.txt", i + 1))?;
        day(input.trim_end());
    }

    Ok(())
}
