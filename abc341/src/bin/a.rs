use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ret = (1..=n).fold(String::from("1"), |acc, _| acc + "01");
    println!("{}", ret);
}
