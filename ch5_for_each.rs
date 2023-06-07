fn main() {
    let notes = [60, 64, 67, 64, 60, 64, 67, 72];
    for no in notes {
        // ここでnoを書き出す
        println!("{}", no);
    }

    println!("");
    notes.iter().for_each(|no| { println!("{}", *no); });
}

