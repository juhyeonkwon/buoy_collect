//test
#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use crate::routes;

    use dotenv::dotenv;
    use chrono::prelude::*;


    #[actix_web::test]
    async fn send_data_test() {
        dotenv().ok();

        let now : DateTime<Local> = Local::now();
        let now_str = now.to_string();   

        let data = routes::collect::Buoy {
            time: String::from(&now_str[0..19]),
            model: String::from("buoy_1"),
            lat: 32.23123,
            lon: 128.24224,
            w_temp: 2.0,
            salinity: 34.0,
            height: 32.0,
            weight: 33.0,
        };

        let mut app = test::init_service(App::new().service(routes::collect::get_data)).await;

        let resp = test::TestRequest::post()
            .uri("/send")
            .set_form(&data)
            .send_request(&mut app)
            .await;

        // let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
