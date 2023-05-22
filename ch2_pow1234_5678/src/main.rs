// BigIntを使うための宣言
use num_bigint::BigInt;

fn main() {
    let v = BigInt::from(1234);
    // 5678乗を計算
    println!("{}", v.pow(5678));
}
