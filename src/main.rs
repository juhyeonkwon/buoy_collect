extern crate dotenv;

mod db;
mod routes;
mod test;

use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Server run port 3123!");

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/files", "./static"))
            .service(routes::collect::get_data)
            .service(routes::collect::index)
            .service(routes::collect::save_file)
    })
    .bind(("127.0.0.1", 3123))?
    .run()
    .await
}
