// mutableでない変数を変更しようとしてエラーになる例
fn main() {
    let a = 100;
    a = a + 1;
    println!("a is {}", a);
}

