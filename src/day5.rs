use std::ops::RangeInclusive;

pub fn part1(input: &str) -> u64 {
    let (ranges, ids) = input.trim_end().split_once("\n\n").unwrap();
    let ranges = FreshRanges::new(ranges);
    let ids = Ids::new(ids);
    ids.ids.iter().filter(|&&id| ranges.is_fresh(id)).count() as u64
}

pub fn part2(input: &str) -> u64 {
    let (ranges, _) = input.trim_end().split_once("\n\n").unwrap();
    let mut ranges = FreshRanges::new(ranges);
    ranges.coalesce();
    ranges.ranges.iter().map(|r| r.clone().count() as u64).sum()
}

struct FreshRanges {
    ranges: Vec<RangeInclusive<u64>>,
}

impl FreshRanges {
    pub fn new(input: &str) -> Self {
        Self {
            ranges: input
                .split('\n')
                .map(|r| {
                    let (lo, hi) = r.split_once('-').unwrap();
                    lo.parse::<u64>().unwrap()..=hi.parse::<u64>().unwrap()
                })
                .collect(),
        }
    }

    pub fn is_fresh(&self, id: u64) -> bool {
        self.ranges.iter().any(|range| range.contains(&id))
    }

    /// Collapse ranges so all ranges are independent
    pub fn coalesce(&mut self) {
        'outer: while !self.independent() {
            let victim = self.ranges.remove(0);
            for range in &mut self.ranges {
                if let Some(merged) = merge_ranges(&victim, range) {
                    *range = merged;
                    continue 'outer;
                }
            }
            self.ranges.push(victim);
        }
    }

    fn independent(&self) -> bool {
        for i in 0..self.ranges.len() {
            for j in 0..self.ranges.len() {
                if j == i {
                    continue;
                }
                if ranges_overlap(&self.ranges[i], &self.ranges[j]) {
                    return false;
                }
            }
        }
        return true;
    }
}

fn merge_ranges(
    left: &RangeInclusive<u64>,
    right: &RangeInclusive<u64>,
) -> Option<RangeInclusive<u64>> {
    ranges_overlap(left, right).then(|| {
        (std::cmp::min(*left.start(), *right.start()))..=(std::cmp::max(*left.end(), *right.end()))
    })
}

fn ranges_overlap(left: &RangeInclusive<u64>, right: &RangeInclusive<u64>) -> bool {
    let ls = *left.start();
    let le = *left.end();
    let rs = *right.start();
    let re = *right.end();
    ls <= re && rs <= le
}

struct Ids {
    ids: Vec<u64>,
}

impl Ids {
    fn new(input: &str) -> Self {
        Self {
            ids: input
                .split('\n')
                .map(|id| id.parse::<u64>().unwrap())
                .collect(),
        }
    }
}
