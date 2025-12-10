use crate::utils::parse_num_array;

pub fn part1(input: &str) -> u64 {
    let mut total = 0;
    for_each_bank(input.as_bytes(), |bank| {
        let result = largest_joltage(bank);
        total += result
    });
    total
}

pub fn part2(input: &str) -> u64 {
    let mut total = 0;
    for_each_bank(input.as_bytes(), |bank| {
        let result = largest_joltage_12(bank);
        println!("For {}, {result}", String::from_utf8_lossy(bank));
        total += result
    });
    total
}

fn for_each_bank<F>(mut input: &[u8], mut f: F)
where
    F: FnMut(&[u8]),
{
    let per_line = input.iter().position(|&c| c == b'\n').unwrap();
    while !input.is_empty() {
        // println!("input = `{}`", String::from_utf8_lossy(input));
        let Some((line, [_, rest @ ..])) = input.split_at_checked(per_line) else {
            unreachable!()
        };
        // println!("line = `{}`", String::from_utf8_lossy(line));
        f(line);
        input = rest;
    }
}

fn largest_joltage(bank: &[u8]) -> u64 {
    let [mut hi, mut lo, ref rest @ .., last] = *bank else {
        unreachable!()
    };
    if lo > hi {
        hi = lo;
        lo = last;
    }
    for &c in rest {
        if c > hi {
            hi = c;
            lo = last;
        } else if c > lo {
            lo = c;
        }
    }
    if last > lo {
        lo = last;
    }
    parse_num_array::<_, u8>(&[hi, lo]) as u64
}

fn largest_joltage_12(bank: &[u8]) -> u64 {
    println!("Orig bank: {}", String::from_utf8_lossy(bank));
    let mut positions = Vec::new();
    for i in (0..12).rev() {
        let last_pos = positions.last().copied().unwrap_or(0);
        let pos = find(bank, last_pos, i).unwrap();
        positions.push(pos);
    }
    positions
        .into_iter()
        .map(|pos| bank[pos] - b'0')
        .fold(0, |acc, x| acc * 10 + x as u64)
}

fn find(bank: &[u8], ignore: usize, remaining: usize) -> Option<usize> {
    println!("Bank is {}", String::from_utf8_lossy(bank));

    let potential = bank.get(ignore + 1..bank.len() - remaining)?;
    let mut best_val_pos = None;
    for (i, &val) in potential.iter().enumerate().skip(ignore) {
        if let Some((best_val, best_pos)) = &mut best_val_pos {
            if val > *best_val {
                *best_val = val;
                *best_pos = i;
            }
        } else {
            best_val_pos = Some((val, i));
        }
    }

    println!(
        "Best val/pos is {:?}",
        best_val_pos.map(|(v, p)| (v - b'0', p))
    );
    best_val_pos.map(|(_, p)| p + ignore)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day3_2() {
        // assert_eq!(largest_joltage_12(b"987654321111111"), 987654321111);
        // assert_eq!(largest_joltage_12(b"811111111111119"), 811111111119);
        assert_eq!(largest_joltage_12(b"234234234234278"), 434234234278);
        assert_eq!(largest_joltage_12(b"818181911112111"), 888911112111);
    }
}
