fn main() {
    let s = "苦しむ人にはどの日も悪い日である。";
    // 文字列の置換
    let s2 = s.replace("苦しむ人", "陽気な人");
    let s3 = s2.replace("悪い日", "宴会");
    // 置換前と置換後を表示
    println!("置換前: {}\n置換後: {}", s, s3);

    // 今回、&str型に対するreplace()適用を行った。
    // &strは値の変更ができないため、&strに対してreplace()を適用すると戻り値はStringとなる。
    // よって、変数s2とs3はString型。
}

