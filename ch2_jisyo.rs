use std::fs::File;
use std::io::{BufRead, BufReader}; // BufReadはlinesメソッドを使うために必要

fn main() {
    // 辞書ファイルの指定
    let dicfile = "ejdict-hand-utf8.txt";

    // コマンドライン引数をベクターに入れる
    let args: Vec<String> = std::env::args().collect();

    // 引数のチェック
    if args.len() < 2 {
        println!("[USAGE] jisyo word");
        return;
    }
    // 指定された単語の取得
    let word = &args[1];

    // ファイルを開く
    let fp = File::open(dicfile).unwrap();
    // BufReaderで一行ずつ読む
    let reader = BufReader::new(fp);
    for line in reader.lines() {
        // 実際に一行取り出す
        let line = line.unwrap();
        // 行に単語が含まれるか検索
        if line.find(word) == None { continue; }
        println!("{}", line);
    }
}

