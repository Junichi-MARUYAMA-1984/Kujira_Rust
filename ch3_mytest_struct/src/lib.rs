// 商品を表す構造体を定義
#[derive(Debug, PartialEq)]
struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    use super::*; // モジュールの外側の要素を利用

    #[test]
    fn item_test() {
        // 構造体を初期化
        let apple1 = GItem{
            name: String::from("リンゴ"),
            price: 2400,
        };
        let mut apple2 = GItem{
            name: "リンゴ".to_string(),
            price: 0,
        };
        apple2.price = 2400;

        // 構造体のフィールドを比較
        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);

        // 構造体全体を直接比較。GItemのアトリビュートにPartialEqを指定しているので可能になる。
        assert_eq!(apple1, apple2);
    }
}
