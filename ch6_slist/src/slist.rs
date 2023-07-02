// 単方向リストの要素1つを表す構造体
pub struct Node {
    data: isize,
    link: Option<Box<Node>>,
}

// 単方向リストをまとめる構造体
pub struct List {
    head: Option<Box<Node>>,
}
// List構造体のメソッドを定義
impl List {
    // コンストラクタ
    pub fn new() -> Self {
        Self { head: None }
    }
    // 先頭に値を追加
    pub fn unshift(&mut self, v: isize) {
        let new_node = Node { data: v, link: self.head.take() };
        // 所有権移動にtake()をわざわざ使っているのは、
        // selfが&mut selfであり所有権移動ができないため。
        // Option::take()メソッドを用いて実態を取り出している。
        self.head = Some(Box::new(new_node));
    }
    // 末尾に値を追加
    pub fn push(&mut self, v: isize) {
        // 新規の値
        let new_node = Node { data: v, link: None };
        match self.head {
            None => self.head = Some(Box::new(new_node)),
            Some(ref mut head) => {
                // 末尾のノードを探して新規ノードをリンクする
                let mut p = head;
                loop {
                    match p.link {
                        None => { // 末尾を見つけた
                            p.link = Some(Box::new(new_node));
                            break;
                        },
                        Some(ref mut next) => p = next,
                    }
                }
            }
        }
    }
    // 任意のインデックスの値を返す
    pub fn get(&self, index: isize) -> Option<isize> {
        match self.head {
            None => return None, // リストが空の場合
            Some(ref top) => {
                // 任意のインデックスの値を探す
                let mut p = top;
                let mut i = 0;
                loop {
                    if i == index { // 見つけた
                        return Some(p.data);
                    }
                    match p.link {
                        None => return None,
                        Some(ref link) => p = link,
                    }
                    i += 1;
                }
            }
        }
    }
}

