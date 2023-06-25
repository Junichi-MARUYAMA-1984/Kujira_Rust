fn main() {
    // ヒープに100を確保してポインタを返す
    let x_box = Box::new(100);
    // 参照外し（デリファレンス）すれば値を取り出せる
    let x_val = *x_box;
    println!("{}", x_val);
}

