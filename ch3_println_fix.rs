fn main() {
    let s = "気前よく与えて豊かになる人がいる".to_string();
    println!("{}", s); // 所有権は移動しない。println!()はそのような設計になっている。
    println!("{}", s); // なので、二回連続実行も問題なく可能。
}

