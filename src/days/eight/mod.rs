use actix_web::{get, web, HttpResponse, Responder};
use serde_json::Value;

const G: f64 = 9.825;
const HEIGHT: f64 = 10.0;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1);
    cfg.service(part_2);
}

async fn get_weight(id: u32) -> u64 {
    let body = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{id}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let json: Value = serde_json::from_str(body.as_str()).unwrap();
    let weight = json["weight"].as_u64().unwrap();

    weight / 10
}

#[get("/8/weight/{pokedex_number}")]
async fn part_1(pokedex_number: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", get_weight(pokedex_number.into_inner()).await))
}

#[get("/8/drop/{pokedex_number}")]
async fn part_2(pokedex_number: web::Path<u32>) -> impl Responder {
    let weight = get_weight(pokedex_number.into_inner()).await as f64;
    let momentum = (2.0 * G * HEIGHT).sqrt() * weight;
    HttpResponse::Ok().body(format!("{}", momentum))
}

#[cfg(test)]
mod tests {
    use actix_web::{body, test, App};

    #[actix_web::test]
    async fn test_part_1() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let req = test::TestRequest::get().uri("/8/weight/25").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();
        assert_eq!(bytes, "6");
    }

    #[actix_web::test]
    async fn test_part_2() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let req = test::TestRequest::get().uri("/8/drop/25").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();
        assert_eq!(bytes, "84.10707461325713");
    }
}
