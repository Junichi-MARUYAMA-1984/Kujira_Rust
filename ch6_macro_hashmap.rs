// HashMapを初期化するマクロを定義
macro_rules! map_init {
    ($($key: expr => $val: expr),*) => {{
        // HashMapを生成
        let mut tmp = std::collections::HashMap::new();
        $(
            // 繰り返し値を挿入
            tmp.insert($key, $val);
        )*
        tmp // オブジェクトを返す
    }}
}

fn main() {
    // マクロを利用してHashMapを初期化
    let week = map_init![
        "mon" => "月曜",
        "tue" => "火曜",
        "wed" => "水曜",
        "thu" => "木曜",
        "fri" => "金曜",
        "sat" => "土曜",
        "sun" => "日曜"
    ];
    println!("mon={}", week["mon"]);
    println!("wed={}", week["wed"]);
}

