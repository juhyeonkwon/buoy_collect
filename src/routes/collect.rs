use crate::db::redis_lib::connect_redis;
use actix_web::{post, web, HttpResponse, Responder};

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
