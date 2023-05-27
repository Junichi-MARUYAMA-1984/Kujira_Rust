// &'static strのみ指定できる関数
fn echo(s: &'static str) {
    println!("{}", s);
}

fn main() {
    // 文字列リテラル(ライフタイムを明示して書くと&'static str型)を引数に指定。
    // これはecho()が受け入れる型と合っているのでOK。
    echo("愚かな人でも黙っていると");
    echo("賢いと見られる");

    // 以下のコメント部分は失敗する。
    // let s = String::from("テスト");
    // echo(&s);
}

