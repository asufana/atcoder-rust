use proconio::input;

fn main() {
    input! {
        (n, a, b): (u32, u32, u32)
    }

    let ret = (1..n + 1)
        .filter(|x| {
            let sum = x
                .to_string()
                .chars()
                .map(|s| s.to_digit(10).unwrap())
                .sum::<u32>();
            a <= sum && sum <= b
        })
        .sum::<u32>();
    println!("{}", ret);
}
