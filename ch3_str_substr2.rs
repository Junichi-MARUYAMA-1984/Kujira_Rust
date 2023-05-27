fn main() {
    let pr = "知恵は武器よりも価値がある。";

    // 先頭2文字の部分文字列を得る
    let sub3: String = pr.chars().take(2).collect();
    println!("先頭2文字: {}", sub3);

    // 「武器」の部分文字列を得る
    let pr_chars: Vec<char> = pr.chars().collect();
    // (0から数えて)3から4文字目
    let sub_chars = &pr_chars[3..=4]; // スライス
    // スライスを文字列に変換
    let sub4: String = sub_chars.into_iter().collect();
    println!("4-5文字目: {}", sub4);

    // 試し
    let sub5: String = pr.chars().collect();
    println!("{}", sub5);
    for c in pr_chars.iter() { print!("{}", c); }
}

