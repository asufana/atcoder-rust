use proconio::input;

pub fn main() {
    input! {
        a: usize,
        b: usize
    }

    if a * b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}