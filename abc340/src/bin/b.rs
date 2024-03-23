use proconio::input;

fn main() {
    input! {
        q: usize,
        qs: [(usize, usize); q]
    }

    qs.iter().fold(Vec::new(), |mut acc, (i, x)| match i {
        1 => {
            acc.push(x);
            acc
        }
        _ => {
            println!("{}", acc.get(acc.len() - x).unwrap());
            acc
        }
    });
}
