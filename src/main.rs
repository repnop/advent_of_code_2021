mod day01;
mod day02;
mod day03;
mod day04;

static DAYS: &[fn(&str)] = &[day01::run, day02::run, day03::run, day04::run];

fn rough_time<T>(f: impl FnOnce() -> T) -> (std::time::Duration, T) {
    let now = std::time::Instant::now();
    core::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    let ret = f();
    core::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    (now.elapsed(), ret)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (i, day) in DAYS.iter().enumerate() {
        let input = match std::fs::read_to_string(format!("input/day{:>02}.txt", i + 1)) {
            Ok(input) => input,
            Err(_) => continue,
        };
        day(input.trim_end());
    }

    Ok(())
}
