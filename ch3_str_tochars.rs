fn main() {
    let pr = "窮鼠猫を噛む";
    // 1文字ずつ表示
    for c in pr.chars() {
        print!("[{}]", c);
    }
    // 文字数を調べる
    println!("\n文字数={}字", pr.chars().count());

    // Vec<char>に変換して処理する
    let pr_chars: Vec<char> = pr.chars().collect();
    for c in pr_chars.iter() {
        print!("({})", c);
    }
    println!("\n文字数={}字", pr_chars.len()); // Vec<char>のlen()は単純に文字数を返す
}

