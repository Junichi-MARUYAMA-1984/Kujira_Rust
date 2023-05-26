// 問題のあるプログラム
fn main() {
    let s = "気前よく与えて豊かになる人がいる".to_string();
    echo(s); // 所有権が移動してしまう
    println!("{}", s); // ここで変数sは既に空なのでエラーになる
}

fn echo(s: String) {
    println!("{}", s);
}

