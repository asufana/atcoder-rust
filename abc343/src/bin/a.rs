use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize)
    }

    let ret = if (a, b) == (0, 0) { 1 } else { 0 };
    println!("{}", ret);
}
