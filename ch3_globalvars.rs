// グローバル変数を定義
static mut TAX: f32 = 0.1;

fn main() {
    // 安全でないことを宣言
    unsafe {
        // ミュータブルなstatic変数を使う
        println!("Price: {}", TAX * 300.0);
        // staticな変数を変更する
        TAX = 0.08;
        println!("Price: {}", TAX * 300.0);
    }
}

