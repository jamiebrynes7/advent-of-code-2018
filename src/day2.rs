use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines()
        .map(|l| {
            l.trim().to_string()
        }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let (mut num_twice, mut num_thrice) = (0,0);
    for id in input {
        let (two, three) = hash_id(id);
        num_twice += two;
        num_thrice += three;
    }

    num_twice * num_thrice
}

fn hash_id(id: &str) -> (i32, i32) {
    let mut character_freq : HashMap<char, i32> = HashMap::new();

    for c in id.chars() {
        *character_freq.entry(c).or_insert(0) += 1;
    }

    let mut contains_twice = 0;
    let mut contains_thrice = 0;

    for (_, v) in character_freq.iter() {
        match v {
            2 => contains_twice = 1,
            3 => contains_thrice = 1,
            _ => {}
        };
    }

    (contains_twice, contains_thrice)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[String]) -> String {
    for id in input {
        for id2 in input {
            if differ_by_one(id, id2) {
                return find_common(id, id2);
            }
        }
    }

    panic!("Could not find strings that only differ by one")
}

fn differ_by_one(s1: &str, s2: &str) -> bool {
    let mut differ_count = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            differ_count += 1;
        }
    };

    differ_count == 1
}

fn find_common(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect::<String>()
}