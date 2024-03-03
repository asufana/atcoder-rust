use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    
    let ret = a.iter().map(|&x| x.trailing_zeros()).min().unwrap();
    println!("{}", ret); 
}
