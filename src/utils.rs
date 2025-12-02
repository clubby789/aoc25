use std::ops::{Add, Mul};

pub fn parse_num_array<const N: usize, T>(input: &[u8; N]) -> T
where
    T: Default + From<u8> + Mul<Output = T> + Add<Output = T>,
{
    debug_assert!(input.iter().all(u8::is_ascii_digit));
    input.iter().fold(T::default(), |acc, &b| {
        let casted: T = (b - b'0').into();
        (acc * 10.into()) + casted
    })
}

pub fn parse_num<T>(input: &[u8]) -> T
where
    T: Default + From<u8> + Mul<Output = T> + Add<Output = T>,
{
    debug_assert!(input.iter().all(u8::is_ascii_digit));
    input.iter().fold(T::default(), |acc, &b| {
        let casted: T = (b - b'0').into();
        (acc * 10.into()) + casted
    })
}
