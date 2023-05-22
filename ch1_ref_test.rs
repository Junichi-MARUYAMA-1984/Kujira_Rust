fn main() {
    // 変数vを10にする
    let mut v = 10;

    // 関数を呼び出す
    set_value(&mut v);

    // 変数vの値は？
    println!("v={}", v);
}

// 引数の値を100に変更する関数
fn set_value(arg: &mut u32) {
    *arg = 100;
}

