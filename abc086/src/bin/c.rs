use proconio::input;

fn main() {
    input! {
        n: usize,
        ts: [(i32, i32, i32); n],
    }

    let mut vec = ts.to_vec();
    vec.insert(0, (0, 0, 0));

    let ret = vec.windows(2).all(|v| {
        let (t, x, y) = v[0]; //(0,0,0)
        let (tn, xn, yn) = v[1]; //(3,1,2)

        let time = tn - t; //3 経過時間
        let dist = (xn - x).abs() + (yn - y).abs(); //1+2=3 絶対移動距離

        dist <= time //3<=3 絶対移動距離は経過時間と同じかそれ以下
            && time % 2 == dist % 2 //3%2==3%2 パリティ（偶奇）が一致していること
    });
    println!("{}", if ret { "Yes" } else { "No" });
}
