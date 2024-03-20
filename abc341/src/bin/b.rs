use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        st: [(usize, usize); n - 1]
    }

    let mut st = st.to_vec();
    st.push((1, 1));
    let ast = a.iter().zip(st).collect::<Vec<(&usize, (usize, usize))>>();

    let ret = ast.iter().fold(0, |acc, (&a, (s, t))| (a + acc) / s * t);
    println!("{:?}", ret);
}
