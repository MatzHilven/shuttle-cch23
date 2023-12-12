use std::collections::HashMap;
use std::time::Instant;

use actix_web::{get, HttpResponse, post, Responder, web};
use chrono::{Datelike, DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use ulid::serde::ulid_as_uuid;
use ulid::Ulid;

struct AppState {
    strings: Mutex<HashMap<String, Instant>>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::Data::new(AppState {
        strings: Mutex::new(HashMap::new()),
    }));
    cfg.service(part_1_save);
    cfg.service(part_1_load);
    cfg.service(part_2);
    cfg.service(part_3);
}

#[post("/12/save/{string}")]
async fn part_1_save(string: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let mut strings = state.strings.lock().await;
    strings.insert(string.into_inner(), Instant::now());
    HttpResponse::Ok()
}

#[get("/12/load/{string}")]
async fn part_1_load(string: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let strings = state.strings.lock().await;
    let time = strings.get(&string.into_inner());
    match time {
        Some(time) => time.elapsed().as_secs().to_string(),
        None => "0".to_string(),
    }
}


#[derive(Serialize, Deserialize)]
struct UlidUuid(#[serde(serialize_with = "ulid_as_uuid::serialize")] Ulid);

#[post("/12/ulids")]
async fn part_2(ulids: web::Json<Vec<UlidUuid>>) -> impl Responder {
    let mut ulids = ulids.into_inner();
    ulids.reverse();

    HttpResponse::Ok().json(ulids)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Response {
    #[serde(rename = "christmas eve")]
    christmas_eve: u32,
    #[serde(rename = "weekday")]
    weekday_count: u32,
    #[serde(rename = "in the future")]
    future_count: u32,
    #[serde(rename = "LSB is 1")]
    lsb_count: u32,
}

#[post("/12/ulids/{weekday}")]
async fn part_3(weekday: web::Path<u32>, ulids: web::Json<Vec<Ulid>>) -> impl Responder {
    let weekday = weekday.into_inner();
    let mut christmas_eve = 0;
    let mut weekday_count = 0;
    let mut future_count = 0;
    let mut lsb_count = 0;

    for ulid in ulids.into_inner() {
        let date: DateTime<Utc> = ulid.datetime().into();
        if date.month() == 12 && date.day() == 24 {
            christmas_eve += 1;
        }
        if date.weekday().num_days_from_monday() == weekday {
            weekday_count += 1;
        }
        if date > chrono::Utc::now() {
            future_count += 1;
        }
        if ulid.random() & 1 == 1 {
            lsb_count += 1;
        }
    }

    HttpResponse::Ok().json(
        Response {
            christmas_eve,
            weekday_count,
            future_count,
            lsb_count,
        }
    )
}

#[cfg(test)]
mod tests {
    use actix_web::{App, body, test};

    #[actix_web::test]
    async fn test_part_1() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let req = test::TestRequest::post()
            .uri("/12/save/packet20231212")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        // wait
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let req = test::TestRequest::get()
            .uri("/12/load/packet20231212")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();
        let response = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(response, "5");
    }

    #[actix_web::test]
    async fn test_part_2() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let ulids = vec![
            "01BJQ0E1C3Z56ABCD0E11HYX4M".to_string(),
            "01BJQ0E1C3Z56ABCD0E11HYX5N".to_string(),
            "01BJQ0E1C3Z56ABCD0E11HYX6Q".to_string(),
            "01BJQ0E1C3Z56ABCD0E11HYX7R".to_string(),
            "01BJQ0E1C3Z56ABCD0E11HYX8P".to_string(),
        ];

        let req = test::TestRequest::post()
            .uri("/12/ulids")
            .set_json(&ulids)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();
        let response: Vec<String> = serde_json::from_slice(&bytes).unwrap();

        let valid_resp = vec![
            "015cae07-0583-f94c-a5b1-a070431f7516".to_string(),
            "015cae07-0583-f94c-a5b1-a070431f74f8".to_string(),
            "015cae07-0583-f94c-a5b1-a070431f74d7".to_string(),
            "015cae07-0583-f94c-a5b1-a070431f74b5".to_string(),
            "015cae07-0583-f94c-a5b1-a070431f7494".to_string(),
        ];

        assert_eq!(response, valid_resp);
    }

    #[actix_web::test]
    async fn test_part_3() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let ulids = vec![
            "00WEGGF0G0J5HEYXS3D7RWZGV8".to_string(),
            "76EP4G39R8JD1N8AQNYDVJBRCF".to_string(),
            "018CJ7KMG0051CDCS3B7BFJ3AK".to_string(),
            "00Y986KPG0AMGB78RD45E9109K".to_string(),
            "010451HTG0NYWMPWCEXG6AJ8F2".to_string(),
            "01HH9SJEG0KY16H81S3N1BMXM4".to_string(),
            "01HH9SJEG0P9M22Z9VGHH9C8CX".to_string(),
            "017F8YY0G0NQA16HHC2QT5JD6X".to_string(),
            "03QCPC7P003V1NND3B3QJW72QJ".to_string()
        ];

        let req = test::TestRequest::post()
            .uri("/12/ulids/5")
            .set_json(&ulids)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();
        let response: super::Response = serde_json::from_slice(&bytes).unwrap();

        let valid_resp = super::Response {
            christmas_eve:3,
            weekday_count: 1,
            future_count: 2,
            lsb_count: 5,
        };

        assert_eq!(response, valid_resp);
    }
}
