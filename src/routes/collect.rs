use crate::db::redis_lib::connect_redis;
use actix_web::{post, get, web, HttpResponse, Error, Responder};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use uuid::Uuid;
use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Buoy {
    pub time: String,
    pub model: String,
    pub lat: f32,
    pub lon: f32,
    pub w_temp: f32,
    pub salinity: f32,
    pub height: f32,
    pub weight: f32,
}

#[post("/send")]
pub async fn get_data(data: web::Form<Buoy>) -> impl Responder {

    let plane_text: String = serde_json::to_string(&data).expect("error occured in transform Json");

    //키 생성
    let key = format!("{}_{}", &data.time[0..10], &data.time[11..13]);

    //redis
    let mut conn = connect_redis();
    let _: () = redis::cmd("RPUSH")
        .arg(key)
        .arg(plane_text)
        .query(&mut conn)
        .expect("error");

    // let _ : () = conn.lpush(time, plane_text).expect("error");

    HttpResponse::Ok().body("ok")
}

#[post("/bulk")]
pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = format!("./tmp/{}", filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}

#[get("/file")]
pub async fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/bulk" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}