// 構造体Personの定義
struct Person {
    name: String,
    age: i32,
}
// Personのメソッドを定義
impl Person {
    // Personを生成する関連関数を定義。関連関数とは&selfを引数に取らないimplブロック内関数。
    fn new(name: String, age: i32) -> Self {
        Person{ name, age } // 引数名とフィールド変数名が同じ場合、name: nameを省略できる。
    }
}

fn main() {
    // 関連関数newを使ってオブジェクトを生成
    let taro = Person::new("太郎".to_string(), 18);
    // フィールドを確認
    println!("{}さんは{}才。", taro.name, taro.age);
}

