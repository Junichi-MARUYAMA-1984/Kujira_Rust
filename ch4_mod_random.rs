// randomモジュールを宣言
mod random {
    // linearモジュールを宣言
    pub mod linear {
        use std::num::Wrapping;
        // 線形合同法で乱数を生成
        pub fn rand(seed: &mut u32) -> u32 {
            let (a, c) = (134775813u32, 12345u32);
            *seed = (Wrapping(*seed) * Wrapping(a) + 
                     Wrapping(c)).0; // Wrapping<u32>型からu32型を取り出すには.0を付ける。
            *seed
        }
    }
    // xorshiftモジュールを宣言
    pub mod xorshift {
        // XorShiftで乱数を生成
        pub fn rand(seed: &mut u32) -> u32 {
            let mut y = *seed;
            y ^= y << 13;
            y ^= y >> 17;
            y ^= y << 5;
            *seed = y;
            y
        }
    }
}

// モジュールの利用を宣言
use random::{linear, xorshift};
fn main() {
    // それぞれ10回乱数を生成して表示
    let mut seed1 = 12345u32;
    let mut seed2 = 12345u32;
    for i in 0..10 {
        let r1 = linear::rand(&mut seed1) % 6 + 1;
        let r2 = xorshift::rand(&mut seed2) % 6 + 1;
        println!("L:{}回目= {}, {}", i, r1, r2);
    }
}

