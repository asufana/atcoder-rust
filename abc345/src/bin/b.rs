use proconio::input;

fn main() {
    input! {
        x: i64
    }

    //切り上げ除算
    //https://atcoder.jp/contests/abc345/editorial/9562
    //rustの場合、xが負の場合小数点以下が切り捨てられる
    let ret = if x >= 0 { (x + 10 - 1) / 10 } else { x / 10 };
    println!("{}", ret);
}
