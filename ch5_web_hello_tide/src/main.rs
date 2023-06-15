// サーバアドレスとポートを指定
const SERVER_ADDR: &str = "127.0.0.1:8888";

// メイン関数
#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("http://{}/", SERVER_ADDR);
    // Tideのオブジェクトを生成
    let mut app = tide::new();
    // ルーティングを指定
    app.at("/").get(|_| async { Ok("Hello, World!") });
    // サーバを起動
    app.listen(SERVER_ADDR).await?;
    Ok(())
}

