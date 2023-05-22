fn main() {
    let s = "3.1415"; // 文字列を指定
    // f64型に変換
    let num = s.parse::<f64>().expect("変換に失敗");
    // 変換した値を書式に合わせて表示
    println!("{:.2}", num); // 結果→3.14
}
