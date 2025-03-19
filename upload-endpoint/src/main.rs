use std::{fs::File, io::Write};

use axum::{extract::Multipart, response::Html, routing::get, Router};

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../src/public/index.html"))
}

async fn hello() -> Html<&'static str> {
    Html(std::include_str!("../src/public/welcome.html"))
}

pub async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("failed to extract field")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }

        let file_name = field.file_name().unwrap();

        println!("Got file {}", file_name);

        let file_path = format!("../files/{}", file_name);

        let data = field.bytes().await.unwrap();

        let mut file_handle = File::create(file_path).expect("failed to open file handle");

        file_handle.write_all(&data).expect("failed to write data");
    }
}
#[tokio::main]

async fn main() {
    // let adder = |x: i32, y: i32| x + y;
    let app = Router::new()
        .route("/", get(hello))
        .route("/index", get(index).post(upload));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to start listener!");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!");
}
