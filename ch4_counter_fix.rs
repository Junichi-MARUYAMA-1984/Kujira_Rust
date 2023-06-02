// Counter構造体を定義
struct Counter {
    value: i64,
}
impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }
    fn inc(&mut self) {
        self.value += 1;
        println!("value={}", self.value);
    }
}

// Counter構造体を引数に取る関数
fn count(counter: Option<&mut Counter>) {
    match counter {
        None => return,
        Some(c) => c.inc(),
    };
}

fn main() {
    // Counterオブジェクトを引数に呼ぶ
    let mut a = Counter::new();
    count(Some(&mut a));
    count(Some(&mut a));
    // count(&mut a); // エラー。引数はOption<&mut T>なので、Some()でラップする必要がある。
    // Noneオブジェクトを引数に呼ぶ
    let a = None;
    count(a);
}

