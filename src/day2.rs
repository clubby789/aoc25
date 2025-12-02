use crate::utils::parse_num;

pub fn part1(input: &str) -> either::Either<u64, String> {
    let sum = input
        .trim_end_matches('\n')
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let (start, end) = (
                parse_num::<u64>(start.as_bytes()),
                parse_num::<u64>(end.as_bytes()),
            );
            start..=end
        })
        .filter(|id| invalid_id_part1(*id))
        .sum::<u64>();
    either::Either::Left(sum)
}

fn invalid_id_part1(id: u64) -> bool {
    let ndigits = id.ilog10() + 1;
    if ndigits % 2 != 0 {
        return false;
    }
    let hi_digits = id / (10u64.pow(ndigits / 2));
    let lo_digits = id - (hi_digits * 10u64.pow(ndigits / 2));
    hi_digits == lo_digits
}

pub fn part2(input: &str) -> either::Either<u64, String> {
    let sum = input
        .trim_end_matches('\n')
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let (start, end) = (
                parse_num::<u64>(start.as_bytes()),
                parse_num::<u64>(end.as_bytes()),
            );
            start..=end
        })
        .filter(|id| invalid_id_part2(*id))
        .sum::<u64>();
    either::Either::Left(sum)
}

fn invalid_id_part2(id: u64) -> bool {
    let ndigits = id.ilog10() + 1;
    'seq_lengths: for sequence_length in 1..=ndigits / 2 {
        // `ndigits` is not a multiple of this length
        if ndigits % sequence_length != 0 {
            continue;
        }
        let repeats = ndigits / sequence_length;
        let mut cur = id;
        let maybe_sequence = cur % 10u64.pow(sequence_length);

        for _subseq in 1..repeats {
            cur = cur / 10u64.pow(sequence_length);
            let lo_digits = cur % 10u64.pow(sequence_length);
            if lo_digits != maybe_sequence {
                continue 'seq_lengths;
            }
        }
        return true;
    }
    return false;
}
