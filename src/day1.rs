pub fn part1(input: &str) -> either::Either<u64, String> {
    let result = input
        .lines()
        .scan(50u64, |dial, line| {
            let num = match line.as_bytes() {
                [b'L', num @ ..] => -(parse_num(num) as i64),
                [b'R', num @ ..] => parse_num(num) as i64,
                _ => unreachable!(),
            };
            let new = wrapping_add(*dial, num);
            *dial = new;
            Some(new)
        })
        .filter(|&state| state == 0)
        .count() as u64;
    either::Either::Left(result)
}

fn parse_num(num: &[u8]) -> u64 {
    num.iter().fold(0u64, |acc, &x| {
        debug_assert!(x.is_ascii_digit());
        acc * 10 + (x - b'0') as u64
    })
}

/// Add `diff` to `num`, wrapping at the bounds of (0, 99)
#[inline]
fn wrapping_add(num: u64, diff: i64) -> u64 {
    (num as i64 + diff).rem_euclid(100) as u64
}

pub fn part2(input: &str) -> either::Either<u64, String> {
    let result = input
        .lines()
        .scan(50u64, |dial, line| {
            let num = match line.as_bytes() {
                [b'L', num @ ..] => -(parse_num(num) as i64),
                [b'R', num @ ..] => parse_num(num) as i64,
                _ => unreachable!(),
            };
            let (new, count) = wrapping_add_count(*dial, num);
            *dial = new;
            Some(count)
        })
        .sum::<u64>();
    either::Either::Left(result)
}

/// Same as [`wrapping_add`], but counts the number of times 0 was passed during rotation
#[inline]
fn wrapping_add_count(mut num: u64, mut diff: i64) -> (u64, u64) {
    let mut count = 0;
    while diff != 0 {
        let new = num as i64 + diff.signum();
        num = match new {
            -1 => 99,
            100 => 0,
            _ => {
                debug_assert!((0..=99).contains(&new), "`{new}`");
                new as u64
            }
        };
        diff -= diff.signum();
        if num == 0 {
            count += 1;
        }
    }
    (num as u64, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapping_add() {
        assert_eq!(wrapping_add(5, -10), 95);
        assert_eq!(wrapping_add(95, 5), 0);
        assert_eq!(wrapping_add(50, -68), 82);
        assert_eq!(wrapping_add(82, -30), 52);
        assert_eq!(wrapping_add(52, 48), 0);
    }

    #[test]
    fn test_wrapping_add_count() {
        assert_eq!(wrapping_add_count(50, -68), (82, 1));
        assert_eq!(wrapping_add_count(82, -30), (52, 0));
        assert_eq!(wrapping_add_count(52, 48), (0, 1));
        assert_eq!(wrapping_add_count(0, -5), (95, 0));
        assert_eq!(wrapping_add_count(95, 60), (55, 1));
        assert_eq!(wrapping_add_count(55, -55), (0, 1));
        assert_eq!(wrapping_add_count(0, -1), (99, 0));
        assert_eq!(wrapping_add_count(50, 1000), (50, 10));
        assert_eq!(wrapping_add_count(5, -105), (0, 2));
    }
}
