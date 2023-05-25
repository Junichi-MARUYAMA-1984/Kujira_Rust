// 引数の値を2倍にする関数
fn x2(arg: &mut i32) {
    *arg = *arg * 2;
}

fn main() {
    let mut v = 16;
    x2(&mut v); // 引数を2倍に
    println!("{}", v);
}

