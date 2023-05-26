// 問題のあるプログラム
fn main() {
    let s = "こんにちは"; // 文字列リテラルは&str型。
    println!("{}", s[0]); // 直接n文字目を得ることはできない。String型/&str型はVec<u8>型。
}
