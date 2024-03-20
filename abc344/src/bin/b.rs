use proconio::input;

fn main() {
    fetch();
}

fn fetch() {
    input! {
        a: usize
    }
    if a > 0 {
        fetch();
    }
    println!("{}", a)
}
