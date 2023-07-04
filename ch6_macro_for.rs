// BASICライクなforが書けるマクロを定義
macro_rules! easy_for {
    // for i = 1 to 10 のような場合
    (
        for $i:ident = $from:tt to $to:tt
        $block:block
    ) => {{
        for $i in $from..=$to {
            $block
        }
    }};
    // for i = 1 to 10 step 2 のような場合
    (
        for $i:ident = $from:tt to $to:tt step $step:tt
        $block:block
    ) => {{
        let mut $i = $from;
        loop {
            if $i > $to { break }
            $block
            $i += $step
        }
    }};
}

fn main() {
    // マクロを利用して1から10までの合計を求める
    let mut total = 0;
    easy_for! {
        for i = 1 to 10 {
            total += i;
        }
    }
    println!("{}", total);
    // マクロを利用して0から10まで3刻みで表示する
    easy_for! {
        for i = 0 to 10 step 3 {
            println!("i={}", i);
        }
    }
}
