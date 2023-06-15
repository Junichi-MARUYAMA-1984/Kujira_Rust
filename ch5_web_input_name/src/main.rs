use tide::prelude::*;
const SERVER_ADDR: &str = "127.0.0.1:8888";

// ユーザ情報を表す構造体を定義
#[derive(Deserialize, Serialize)]
struct UserInfo {
    name: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("http://{}/", SERVER_ADDR);
    let mut app = tide::new();
    // ルーティングを指定
    app.at("/").get(|_| async { // ルートにアクセスしたとき
        // HTMLを表示
        Ok(tide::Response::builder(200)
           .content_type(tide::http::mime::HTML)
           .body(format!("{}{}{}{}",
                         "<html><body><form action='hello'>",
                         "name: <input name='name' value='kujira'>",
                         "<input type='submit' value='送信'>",
                         "</form></body></html>"))
           .build())
    });
    // "/hello"にアクセスしたときの処理
    app.at("/hello").get(|req: tide::Request<()>| async move {
        // クエリを解析して構造体に当てはめる
        let user: UserInfo = req.query()?;
        Ok(tide::Response::builder(200)
           .content_type(tide::http::mime::HTML)
           .body(format!("<h1>Hello, {}</h1>", user.name))
           .build())
    });
    // サーバを起動
    app.listen(SERVER_ADDR).await?;
    Ok(())
}

