//test
#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use crate::routes;

    use dotenv::dotenv;
    use chrono;
    use chrono::prelude::*;
    use chrono::Duration;
    
    use serde::{Serialize, Deserialize};
    use rand::prelude::*;

    use crate::db::maria_lib::Buoy;
    use crate::db::maria_lib::DataBase;

    use mysql::*;
    use mysql::prelude::*;

    #[derive(Serialize, Debug)]
    pub struct Modelinfo {
        pub model : String,
        pub group_id : i32,
        pub latitude: f32,
        pub longitude: f32,
    }

    #[actix_web::test]
    // #[test]
    async fn send_data_test() {
        dotenv().ok();

        let mut db = DataBase::init();

        let row = db.conn
        .query_map("SELECT model, group_id, latitude, longitude FROM buoy_model ORDER BY model_idx",|(model, group_id, latitude, longitude)| Modelinfo {model, group_id, latitude, longitude})
        .expect("queery Errror");

        let now : DateTime<Local> = Local::now();
        let now_str = now.to_string();   

        let mut app = test::init_service(App::new().service(routes::collect::get_data)).await;

        for n in 0..100 {
            let mut rng = rand::thread_rng();

            let data = Buoy {
                    time: String::from(&now_str[0..19]),
                    model: String::from(&row[n].model),
                    lat: row[n].latitude,
                    lon: row[n].longitude,
                    w_temp: rng.gen_range(12.5..13.5),
                    salinity: rng.gen_range(28.0..33.0),
                    height: rng.gen_range(8.0..20.0),
                    weight: rng.gen_range(40.0..53.0),
            };
            

            let resp = test::TestRequest::post()
            .uri("/send")
            .set_form(&data)
            .send_request(&mut app)
            .await;

            assert!(resp.status().is_success());

        }

       
        // let data = routes::collect::Buoy {
        //     time: String::from(&now_str[0..19]),
        //     model: String::from("buoy_1"),
        //     lat: 34.7972552,
        //     lon: 128.4642089,
        //     w_temp: 2.0,
        //     salinity: 34.0,
        //     height: 32.0,
        //     weight: 33.0,
        // };

        // let mut app = test::init_service(App::new().service(routes::collect::get_data)).await;

        // let resp = test::TestRequest::post()
        //     .uri("/send")
        //     .set_form(&data)
        //     .send_request(&mut app)
        //     .await;

        // // let resp = test::call_service(&app, req).await;
        // assert!(resp.status().is_success());
    }

    #[actix_web::test]
    // #[test]
    async fn send_data_test2() {
        dotenv().ok();

        let mut db = DataBase::init();

        let row = db.conn
        .query_map("SELECT model, group_id, latitude, longitude FROM buoy_model ORDER BY model_idx",|(model, group_id, latitude, longitude)| Modelinfo {model, group_id, latitude, longitude})
        .expect("queery Errror");

        let now : DateTime<Local> = Local::now();
        let now_str = now.to_string();   

        let mut app = test::init_service(App::new().service(routes::collect::get_data)).await;

        for n in 0..2 {
            let mut rng = rand::thread_rng();

            let data = Buoy {
                    time: String::from(&now_str[0..19]),
                    model: String::from("buoy_104"),
                    lat: row[n].latitude,
                    lon: row[n].longitude,
                    w_temp: rng.gen_range(12.5..13.5),
                    salinity: rng.gen_range(28.0..33.0),
                    height: rng.gen_range(8.0..20.0),
                    weight: rng.gen_range(40.0..53.0),
            };
            

            let resp = test::TestRequest::post()
            .uri("/send")
            .set_form(&data)
            .send_request(&mut app)
            .await;

            assert!(resp.status().is_success());

        }
    }
}
