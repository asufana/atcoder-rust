use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        (n, sum): (i32, i32)
    }

    let (x, y, z) = iproduct!(0..=n, 0..=n)
        .find(|&(x, y)| n - x - y >= 0 && 10000 * x + 5000 * y + 1000 * (n - x - y) == sum)
        .map_or((-1, -1, -1), |(x, y)| (x, y, n - x - y));
    println!("{} {} {}", x, y, z);
}
