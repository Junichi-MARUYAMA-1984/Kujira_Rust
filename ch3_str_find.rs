fn main() {
    let s = "隣の客はよく柿食う客だ";

    // '柿'を検索
    match s.find('柿') {
        Some(i) => println!("柿={}Bytes目", i),
        None => println!("柿はなし"),
    };
    // "バナナ"を検索
    match s.find("バナナ") {
        Some(i) => println!("バナナ={}Bytes目", i),
        None => println!("バナナはなし"),
    };
}
