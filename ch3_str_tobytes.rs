fn main() {
    let pr = "猫に小判"; // prは&str型。
    // 1バイトずつ表示
    for c in pr.bytes() {
        print!("{:2x} ", c);
    }
    // バイト数を得る
    println!("\nバイト数={}Bytes", pr.len()); // &strのlen()はバイト数を返す（文字数ではない）
}

