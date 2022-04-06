extern crate dotenv;

mod db;
mod routes;
mod test;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use actix_files as fs;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hellodsadasdtd world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/files", "./static"))            
            .service(hello)
            .service(echo)
            .service(routes::collect::get_data)
            .route("/hey", web::get().to(manual_hello))
            .route("/sibal", web::get().to(db::maria_lib::get_test))
    })
    .bind(("127.0.0.1", 3123))?
    .run()
    .await
}
