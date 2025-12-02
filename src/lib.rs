mod utils;

use seq_macro::seq;

type AocFn = fn(&str) -> either::Either<u64, String>;

seq! {
    N in 1..=2 {
        #(
            pub mod day~N;
        )*
        pub static FUNCS: &[(AocFn, AocFn)] = &[
            #(
                (day~N::part1 as _, day~N::part2 as _),
            )*
        ];
    }
}
