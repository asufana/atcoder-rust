use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [[usize; n]; n]
    }

    let ret = an
        .iter()
        .map(|an| (1..).zip(an).collect::<Vec<(i32, &usize)>>())
        .map(|an| {
            an.iter()
                .filter(|(_, a)| **a == 1)
                .map(|(i, _)| *i)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    ret.iter().for_each(|an| {
        an.iter().for_each(|a| print!("{a} "));
        println!("");
    });
}
