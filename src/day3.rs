use std::collections::HashMap;

pub struct Claim {
    pub id: u32,
    pub offset: (u32, u32),
    pub dimensions: (u32, u32)
}

impl PartialEq for Claim {
    fn eq(&self, other: &Claim) -> bool {
        self.id == other.id
    }
}

impl Claim {
    pub fn from_line(input: &str) -> Self {
        let mut split = input.split(" @ ");

        let id = split.nth(0).unwrap().replace("#", "").parse::<u32>().unwrap();
        let mut size_iter = split.nth(0).unwrap().split(": ");

        let mut offset_iter = size_iter.nth(0).unwrap().split(",");
        let offset_x = offset_iter.nth(0).unwrap().parse::<u32>().unwrap();
        let offset_y = offset_iter.nth(0).unwrap().parse::<u32>().unwrap();

        let mut dimension_iter = size_iter.nth(0).unwrap().split("x");
        let dim_x = dimension_iter.nth(0).unwrap().parse::<u32>().unwrap();
        let dim_y = dimension_iter.nth(0).unwrap().parse::<u32>().unwrap();

        Claim {
            id,
            offset: (offset_x, offset_y),
            dimensions: (dim_x ,dim_y)
        }
    }

    // Returns (min x, max x, min y, max y)
    pub fn rect(&self) -> (u32, u32, u32, u32) {
        (
            self.offset.0,
            self.offset.0 + self.dimensions.0,
            self.offset.1,
            self.offset.1 + self.dimensions.1
        )
    }

    pub fn does_overlap(&self, other: &Claim) -> bool {
        let (x1s, x2s, y1s, y2s) = self.rect();
        let (x10, x20, y10, y20) = other.rect();

        if x1s > x20 || x10 > x2s {
            return false;
        }

        if y1s > y20 || y10 > y2s {
            return false;
        }

        true
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    input.lines().map(|line| {
        Claim::from_line(line)
    }).collect()
}

#[aoc(day3, part1)]
pub fn solve_part_1(input: &[Claim]) -> u32 {
    let mut map : HashMap<(u32, u32), u32> = HashMap::new();
    let mut hits = 0;

    for claim in input {
        for x in claim.offset.0..(claim.offset.0 + claim.dimensions.0) {
            for y in claim.offset.1..(claim.offset.1 + claim.dimensions.1) {
                let value = map.entry((x,y)).or_insert(0);
                *value += 1;
                if *value == 2 {
                    hits += 1;
                }
            }
        }
    }

    hits
}

#[aoc(day3, part2)]
pub fn solve_part_2(input: &[Claim]) -> u32 {
    for claim in input {
        if input
            .iter()
            .filter(|c| { *c != claim})
            .all(|c| { !claim.does_overlap(c)}) {
            return claim.id;
        }
    }

    0
}
