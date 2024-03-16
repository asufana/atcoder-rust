use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String
    }

    let sorted = (1..)
        .zip(s.chars())
        .sorted_by_key(|&(_, c)| c)
        .collect::<Vec<(usize, char)>>();

    let first = sorted[0];
    let second = sorted[1];
    let last = sorted[sorted.len() - 1];

    let ret = if first.1 != second.1 { first.0 } else { last.0 };
    println!("{}", ret);
}
