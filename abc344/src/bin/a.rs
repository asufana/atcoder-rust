use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String
    }

    let regex = Regex::new(r"(\|.*?\|)").unwrap();
    let ret = regex.replace(&s, "");
    println!("{}", ret);
}
