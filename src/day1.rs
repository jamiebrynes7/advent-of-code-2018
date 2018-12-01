use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines()
        .map(|l| {
            l.trim().parse::<i32>().unwrap()
        }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let mut freq = 0;
    for change in input {
        freq += change;
    }

    freq
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut frequencies : HashSet<i32> = HashSet::new();
    let mut freq = 0;
    'outer: loop {
        for change in input {
            freq += change;
            if !frequencies.insert(freq) {
                break 'outer;
            }
        }
    }

    freq
}