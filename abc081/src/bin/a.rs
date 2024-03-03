use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ret = s.chars().filter(|&x| x=='1').count();
    println!("{}", ret);
}
