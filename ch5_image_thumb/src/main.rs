use image::{self, imageops, GenericImageView};

fn main() {
    // リサイズするサイズ
    let size = 128;
    // コマンドライン引数を得る
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] image_thumb imagefile");
        return;
    }
    // 入力ファイルと出力ファイルを指定
    let infile = String::from(&args[1]);
    let outfile = format!("{}-thumb.png", infile);
    println!("input: {}", infile);
    println!("output: {}", outfile);
    // 画像を読み込む
    let mut img = image::open(infile)
        .expect("画像ファイルが読めません");
    // 画像サイズを得る
    let dim = img.dimensions();
    // 正方形に切り取る
    let w = if dim.0 > dim.1 { dim.1 } else { dim.0 };
    let mut img2 = imageops::crop(&mut img,
                                  (dim.0 - w) / 2, (dim.1 - w) / 2,
                                  w, w).to_image();
    // 指定サイズにリサイズ
    let img3 = imageops::resize(&mut img2, size, size, imageops::Lanczos3);
    // ファイルに保存
    img3.save(outfile).unwrap();
}

