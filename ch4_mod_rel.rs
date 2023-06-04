mod random {
    pub mod linear {
        pub fn rand() -> u32 {
            1
        }
    }
    pub mod xorshift {
        pub fn rand() -> u32 {
            // crate::random::linear::rand()を呼ぶには？
            super::linear::rand()
            // もしくは、こちらでもOK
            // crate::random::linear::rand()
        }
    }
}

fn main() {
    // xorshift::rand()を呼ぶ
    println!("{}", random::xorshift::rand());
}

