use proconio::input;

fn main() {
    input! {
        s: String
    }

    let ret = s
        .chars()
        .fold(String::new(), |mut acc, c| match acc.chars().last() {
            Some(last) => {
                if c == '=' && last == c {
                    acc
                } else {
                    acc.push(c);
                    acc
                }
            }
            None => {
                acc.push(c);
                acc
            }
        });
    println!("{}", if ret == "<=>" { "Yes" } else { "No" });
}
