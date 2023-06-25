// 列挙型でNodeを定義
enum Node {
    Empty,
    Cons(i64, Box<Node>), // タプル型
}

// 列挙型を手軽に使えるよう宣言
use Node::{Empty, Cons};
fn node(v: i64, link: Box<Node>) -> Box<Node> {
    Box::new(Cons(v, link))
}

fn main() {
    // 単方向リストを生成
    let c = node(10, node(20, node(30, Box::new(Empty))));

    // 先頭から各要素をたどって表示
    let mut ptr: &Box<Node> = &c;
    loop {
        // &Box<Node>からNode（へのポインタ）を取り出す
        let cur_node: &Node = &**ptr;
        match cur_node {
            Empty => break,
            Cons(v, link) => {
                println!("{}", v);
                ptr = &link;
            }
        }
    }
}

