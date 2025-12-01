use std::num::NonZeroI16;

use crate::utils::parse_num_array;

pub fn part1(input: &str) -> either::Either<u64, String> {
    let mut count = 0;
    let mut dial = 50i16;
    for_each_line(input, |action| {
        dial += action.0.get();
        if dial.rem_euclid(100) == 0 {
            count += 1;
        }
    });
    either::Either::Left(count)
}

struct Action(NonZeroI16);

impl Action {
    fn peel_from_input(input: &[u8]) -> Option<(Self, &[u8])> {
        let (dir, num, rest) = match *input {
            [dir, a, b, c, b'\n', ref rest @ ..] => (
                dir,
                NonZeroI16::new(parse_num_array(&[a, b, c])).unwrap(),
                rest,
            ),
            [dir, a, b, b'\n', ref rest @ ..] => (
                dir,
                NonZeroI16::new(parse_num_array(&[a, b])).unwrap(),
                rest,
            ),
            [dir, a, b'\n', ref rest @ ..] => {
                (dir, NonZeroI16::new(parse_num_array(&[a])).unwrap(), rest)
            }
            _ => return None,
        };
        if dir == b'L' {
            Some((Action(-num), rest))
        } else {
            Some((Action(num), rest))
        }
    }
}

fn for_each_line<F>(input: &str, mut f: F)
where
    F: FnMut(Action),
{
    let mut input = input.as_bytes();
    while let Some((action, rest)) = Action::peel_from_input(input) {
        input = rest;
        f(action);
    }
}

pub fn part2(input: &str) -> either::Either<u64, String> {
    let mut dial = 50;
    let mut count = 0;
    for_each_line(input, |action| {
        let tmp_count;
        (dial, tmp_count) = wrapping_add_count(dial, action.0);
        count += tmp_count;
    });

    either::Either::Left(count)
}

/// Same as [`wrapping_add`], but counts the number of times 0 was passed during rotation
#[inline]
fn wrapping_add_count(mut num: u16, diff: NonZeroI16) -> (u16, u64) {
    let mut count = 0;
    let mut diff = diff.get();
    while diff != 0 {
        let new = num as i16 + diff.signum();
        num = match new {
            -1 => 99,
            100 => 0,
            _ => {
                debug_assert!((0..=99).contains(&new), "`{new}`");
                new as u16
            }
        };
        diff -= diff.signum();
        if num == 0 {
            count += 1;
        }
    }
    (num, count)
}
