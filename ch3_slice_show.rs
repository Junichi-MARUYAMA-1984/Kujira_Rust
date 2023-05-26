fn main() {
    // 配列を初期化
    let a = [0, 1, 2, 3, 4, 5];
    // aの先頭3要素のスライスを得る
    let a_slice = &a[0..3];
    println!("{:?}", a_slice); // [0, 1, 2]
    // aの3から(5-1)番目の要素を得る
    println!("{:?}", &a[3..5]); // [3, 4]
    // aの4以降のスライスを得る
    println!("{:?}", &a[4..6]); // [4, 5]
}

