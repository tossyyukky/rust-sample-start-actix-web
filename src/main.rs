use actix_web::{server, App, HttpRequest, Responder, Error, FromRequest, Path, State};
use serde_derive::*;

// Deserializeにしたがって抽出されるので型を用意しておく
#[derive(Deserialize)]
struct HelloPath {
    // {name}に対応するフィールドを定義する
    name: String,
}

// アプリケーション情報を保持するデータ型
struct MyApp {
    server_name: String,
}

// State<T>型の引数を取るとデータ型を受け取れる
fn hello_with_state(app: State<MyApp>) -> Result<String, Error> {
    Ok(format!("Hello from {}!", &app.server_name))
}

//fn hello_name(to: Path<HelloPath>) -> impl Responder {
//    format!("Hello {}!", &to.name)
//}
//fn hello_name(req: &HttpRequest) -> Result<String, Error> {
    // FromRequest::extractでデータを抽出する
//    let to = Path::<HelloPath>::extract(req);
    // Path<T>はDeref<Target=T>を実装しているのでそのままHelloPathのように扱える
//    Ok(format!("Hello {}!", &to.name))
//}


//fn hello(req: &HttpRequest) -> impl Responder {
//    let to = req.match_info().get("name").unwrap_or("World");
//    format!("Hello {}!", to)
//}

fn main() {
    server::new(|| {
//        App::new()
        App::with_state(MyApp {
            server_name: "servier with state".into()
        })
//            .resource("/", |r| r.f(hello))
//            .resource("/{name}", |r| r.f(hello))
//            .resource("/{name}", |r| r.with(hello_name))
            .resource("/info", |r| r.with(hello_with_state))
    })
        .bind("localhost:5000")
        .expect("Can not bind to port 5000")
        .run();
}
