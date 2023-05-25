fn main() {
    let g1 = String::from("過ちを見過ごす人は美しい");
    show_message(&g1); // 参照を渡す。関数の引数に参照を渡すことを借用と呼ぶ。
    println!("{}", g1); // OK。所有権は移動していない。
}

fn show_message(message: &String) {
    println!("{}", message);
}

