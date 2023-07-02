use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // ヒープに内部可変性を持ったi32型を確保
    let a = Rc::new(RefCell::new(1000));
    // 参照カウンタを加算
    let b = Rc::clone(&a);
    // 値を変更する
    *b.borrow_mut() += 100;
    // 値が変更された
    println!("{}", a.borrow());
}

