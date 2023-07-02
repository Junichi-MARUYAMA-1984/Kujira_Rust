use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 双方向リストの要素1つを表す構造体
pub struct Node {
    data: isize,
    next: Option<Rc<RefCell<Node>>>, // 強い参照
    prev: Option<Weak<RefCell<Node>>>, // 弱い参照
}

// 双方向リストをまとめる構造体
pub struct List {
    head: Option<Rc<RefCell<Node>>>,
    foot: Option<Rc<RefCell<Node>>>,
}
// List構造体のメソッドを定義
impl List {
    // コンストラクタ
    pub fn new() -> Self {
        Self { head: None, foot: None }
    }
    fn new_node(v: isize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            data: v, next: None, prev: None }))
    }
    // 末尾に値を追加
    pub fn push(&mut self, v: isize) {
        let n = List::new_node(v);
        match self.foot.take() {
            None => {
                self.foot = Some(Rc::clone(&n));
                self.head = Some(n);
            },
            Some(old_foot) => {
                self.foot = Some(Rc::clone(&n));
                n.borrow_mut().prev = 
                    Some(Rc::downgrade(&old_foot));
                old_foot.borrow_mut().next = Some(n);
            },
        }
    }
    // 先頭に値を追加
    pub fn unshift(&mut self, v: isize) {
        let n = List::new_node(v);
        match self.head.take() {
            None => {
                self.foot = Some(Rc::clone(&n));
                self.head = Some(n);
            },
            Some(old_head) => {
                old_head.borrow_mut().prev =
                    Some(Rc::downgrade(&old_head));
                n.borrow_mut().next = Some(old_head);
                self.head = Some(n);
            }
        }
    }
    // イテレータを返すメソッド
    pub fn iter(&mut self) -> ListIter {
        match &self.head {
            None => ListIter { cur: None },
            Some(head) => {
                let head = Rc::clone(&head);
                ListIter { cur: Some(head) }
            },
        }
    }
}

// イテレータのための構造体
pub struct ListIter {
    pub cur: Option<Rc<RefCell<Node>>>,
}
// イテレータの実装
impl Iterator for ListIter { // Iteratorトレイトの実装
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur.take() {
            None => None,
            Some(cur) => {
                let cb = cur.borrow();
                let data = cb.data; // 今回の値を得る
                match &cb.next { // カーソルを次に進める
                    None => self.cur = None,
                    Some(next) => {
                        self.cur = Some(Rc::clone(&next))
                    },
                }
                Some(data)
            }
        }
    }
}

