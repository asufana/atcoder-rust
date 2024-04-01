use proconio::input;

fn main() {
    input! {
        s: String
    }

    let ret = s.split('.').last().unwrap();
    println!("{}", ret);
}
