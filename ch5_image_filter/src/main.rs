use image::{GenericImage, GenericImageView, Rgba};

fn main() {
    // コマンドライン引数を得る
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] image_filter imagefile");
        return;
    }
    // 入力ファイルと出力ファイルを指定
    let infile = args[1].clone();
    let outfile = format!("{}-out.jpg", infile);
    println!("infile={}", infile);
    println!("outfile={}", outfile);
    // 画像ファイルを読む
    let mut img = image::open(infile)
        .expect("画像が読めません。");
    // 画像の幅と高さを得る
    let (w, h) = img.dimensions();
    // 行と列をそれぞれ繰り返す
    for y in 0..h {
        for x in 0..w {
            // ピクセルデータを得る
            let c: Rgba<u8> = img.get_pixel(x, y);
            // ネガポジ反転処理
            let c = Rgba([
                         255 - c[0], // 赤
                         255 - c[1], // 緑
                         255 - c[2], // 青
                         c[3], // 透明度
            ]);
            // ピクセルを設定
            img.put_pixel(x, y, c);
        }
    }
    // 画像を保存
    img.save(outfile).unwrap();
}

