use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
        a: [(usize, usize); q],
    }

    let ps = (1..).zip(p).collect::<Vec<(usize, usize)>>();

    a.iter()
        .map(|&a| solve(ps.clone(), a))
        .for_each(|n| println!("{}", n));
}

fn find(ps: Vec<(usize, usize)>, a: usize) -> (usize, usize) {
    *ps.iter().find(|&&(_, n)| n == a).unwrap()
}

fn solve(ps: Vec<(usize, usize)>, a: (usize, usize)) -> usize {
    let first = find(ps.clone(), a.0);
    let second = find(ps.clone(), a.1);

    if first.0 < second.0 {
        first.1
    } else {
        second.1
    }
}
