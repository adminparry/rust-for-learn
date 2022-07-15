use actix_web::{get, web, App, HttpServer, Responder};


#[get("/")]
async fn greet() -> impl Responder {
    format!("Hello Wrold")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";

    println!("http://{host}:8088");
    let http = HttpServer::new (|| {
        App::new().service(greet)
    });

    http.bind((host, 8088))?.run().await
}