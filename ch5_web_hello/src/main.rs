use actix_web::{web, App, HttpServer, HttpRequest};

// アドレスとポートの指定
const SERVER_ADDR: &str = "127.0.0.1:8888";

// Actix Webのメイン関数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    // HTTPサーバを起動
    HttpServer::new (|| {
        // ルーティングを指定。
        // ルート"/"へのアクセスに対して、index関数をコールバック関数として呼び出す。
        // routeメソッドを指定することで、URLごとに処理を振り分けることが可能。
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(SERVER_ADDR)? // ?演算子はResult型変数のOkの時の値を戻す。match文の省略処理。
    .run()
    .await
}

// 実行される関数
async fn index(req: HttpRequest) -> &'static str {
    println!("request: {:?}", req);
    "Hello, World!"
}

