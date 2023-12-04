use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1);
    cfg.service(part_2);
}

#[derive(Serialize, Deserialize)]
struct Reindeer {
    name: String,
    strength: usize,
}

#[derive(Serialize, Deserialize)]
struct ReindeerContest {
    name: String,
    strength: usize,
    speed: f64,
    height: usize,
    antler_width: usize,
    snow_magic_power: usize,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    buh: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ReindeerContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[post("/4/strength")]
async fn part_1(deers: web::Json<Vec<Reindeer>>) -> impl Responder {
    let total_strength = deers.iter().fold(0, |acc, x| acc + x.strength);
    HttpResponse::Ok().body(total_strength.to_string())
}

#[post("/4/contest")]
async fn part_2(deers: web::Json<Vec<ReindeerContest>>) -> impl Responder {
    let fastest = deers
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .unwrap();
    let tallest = deers
        .iter()
        .max_by(|a, b| a.height.partial_cmp(&b.height).unwrap())
        .unwrap();
    let magician = deers
        .iter()
        .max_by(|a, b| a.snow_magic_power.partial_cmp(&b.snow_magic_power).unwrap())
        .unwrap();
    let consumer = deers
        .iter()
        .max_by(|a, b| a.buh.partial_cmp(&b.buh).unwrap())
        .unwrap();

    let response = ReindeerContestResponse {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    };

    HttpResponse::Ok().body(json!(response).to_string())
}

#[cfg(test)]
mod tests {
    use actix_web::http::header::ContentType;
    use actix_web::{body, test, App};

    #[actix_web::test]
    async fn test_part_1() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let body = r#"[{"name":"Dasher","strength":5},{"name":"Dancer","strength":6},{"name":"Prancer","strength":4},{"name":"Vixen","strength":7}]"#;

        let req = test::TestRequest::post()
            .uri("/4/strength")
            .insert_header(ContentType::json())
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert_eq!(bytes.unwrap(), "22");
    }

    #[actix_web::test]
    async fn test_part_2() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let body = r#"[
    {
      "name": "Dasher",
      "strength": 5,
      "speed": 50.4,
      "height": 80,
      "antler_width": 36,
      "snow_magic_power": 9001,
      "favorite_food": "hay",
      "cAnD13s_3ATeN-yesT3rdAy": 2
    },
    {
      "name": "Dancer",
      "strength": 6,
      "speed": 48.2,
      "height": 65,
      "antler_width": 37,
      "snow_magic_power": 4004,
      "favorite_food": "grass",
      "cAnD13s_3ATeN-yesT3rdAy": 5
    }
  ]"#;

        let req = test::TestRequest::post()
            .uri("/4/contest")
            .insert_header(ContentType::json())
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let valid_response: super::ReindeerContestResponse = serde_json::from_str(
            r#"{
  "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
  "tallest": "Dasher is standing tall with his 36 cm wide antlers",
  "magician": "Dasher could blast you away with a snow magic power of 9001",
  "consumer": "Dancer ate lots of candies, but also some grass"
}"#,
        )
        .unwrap();

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        let response: super::ReindeerContestResponse =
            serde_json::from_slice(&bytes.unwrap()).unwrap();

        assert_eq!(response, valid_response);
    }
}
