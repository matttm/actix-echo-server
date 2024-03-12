use actix_web::{post, App, HttpResponse, HttpServer, Responder};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;
    println!("Listening on {}:{}", host, port);
    HttpServer::new(|| {
        App::new()
            .service(echo)
    })
    .bind((host, port))?
    .run()
    .await
}
