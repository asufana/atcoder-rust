use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String
    }

    let reverse = s.chars().rev().collect::<String>();
    let regex = Regex::new(r"(maerd|remaerd|esare|resare)").unwrap();
    let ret = regex.replace_all(&reverse, "") == "";
    println!("{}", if ret { "YES" } else { "NO " });
}
