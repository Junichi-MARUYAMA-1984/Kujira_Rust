fn main() {
    let val: i32 = 10;
    let val_ptr: *const i32 = &val; // *constを付けることで、Rustの参照をポインタ型に代入できる
    println!("val={}, *val={:?}", val, val_ptr);
}

