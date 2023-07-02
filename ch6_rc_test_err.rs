// 問題のあるプログラム
fn main() {
    // ヒープにi32型の値を確保
    let a_box = Box::new(1000);
    {
        let b_box = a_box; // 所有権が移動してしまう
        println!("{}", b_box);
    }
    println!("{}", a_box); // もはや利用できない
}

