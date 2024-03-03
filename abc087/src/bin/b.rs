use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let ret = iproduct!(0..=a, 0..=b, 0..=c)
        .filter(|&(a, b, c)| 500 * a + 100 * b + 50 * c == x)
        .count();
    println!("{}", ret);
}
