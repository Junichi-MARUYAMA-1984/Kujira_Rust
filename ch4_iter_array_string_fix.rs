fn main() {
    // 配列に文字列を代入
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Tomato".to_string()
    ];
    // 配列を繰り返し画面に表示する
    for a in array.iter() { // iter()は値の参照（&T）のイテレータを返す。所有権移動なし。
        println!("{}", a);
    }
    println!("len={}", array.len()); // OK
}

