use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    //[1,-1,1,-1]配列
    let i: Vec<i32> = std::iter::repeat(vec![1, -1]).take(n).flatten().collect();
    let ret = a
        .iter()
        .sorted()
        .zip(i)
        .map(|(x, i)| x * i)
        .sum::<i32>()
        .abs();
    println!("{}", ret);
}
