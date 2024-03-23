use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (a, b, d): (usize, usize, usize),
    }

    let ret = (a..=b).step_by(d).join(" ");
    println!("{}", ret);
}
