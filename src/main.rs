use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world3!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    NamedFile::open("./src/index.html").unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/app").route("index.html", web::get().to(index))
            )
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}