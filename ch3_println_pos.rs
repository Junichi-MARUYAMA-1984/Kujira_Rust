fn main() {
    let year = 2023;
    let month = 12;
    let day = 1;
    println!("JP:{0}/{1}/{2}", year, month, day); // 日本式
    println!("US:{1}/{2}/{0}", year, month, day); // 米国式
    println!("UK:{2}/{1}/{0}", year, month, day); // 英国式
}

