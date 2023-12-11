use std::collections::HashMap;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use base64::Engine;
use base64::engine::general_purpose;
use serde::{Deserialize, Serialize};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1);
    cfg.service(part_2);
}

#[get("/7/decode")]
async fn part_1(req: HttpRequest) -> impl Responder {
    match req.cookie("recipe") {
        Some(cookie) => {
            let value = cookie.value();
            match base64::engine::general_purpose::STANDARD.decode(value) {
                Ok(decoded) => {
                    let decoded = String::from_utf8(decoded).unwrap();
                    HttpResponse::Ok().body(decoded)
                }
                Err(_) => HttpResponse::BadRequest().body("Invalid base64"),
            }
        }
        None => HttpResponse::BadRequest().body("No cookie found"),
    }
}

#[derive(Serialize, Deserialize)]
struct Recipe {
    flour: u32,
    sugar: u32,
    butter: u32,
    #[serde(rename = "baking powder")]
    baking_powder: u32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u32,
}

#[derive(Serialize, Deserialize)]
struct Pantry {
    flour: u32,
    sugar: u32,
    butter: u32,
    #[serde(rename = "baking powder")]
    baking_powder: u32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u32,
}

#[derive(Serialize, Deserialize)]
struct BakeCookie {
    recipe: Recipe,
    pantry: Pantry,
}

#[derive(Serialize, Deserialize)]
struct BakeCookieResponse {
    cookies: u32,
    pantry: Pantry,
}

#[derive(serde::Deserialize)]
struct Bake {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[get("/7/bake")]
async fn part_2(request: HttpRequest) -> impl Responder {
    let cookie = request.cookie("recipe").unwrap();
    let encoded = cookie.value();

    let decoded = general_purpose::STANDARD.decode(encoded).unwrap();
    let mut bake: Bake = serde_json::from_slice(&decoded).unwrap();

    let cookies = bake
        .recipe
        .iter()
        .map(|(k, &v)| {
            if v == 0 {
                usize::MAX
            } else {
                bake.pantry.get(k).map(|&a| a / v).unwrap_or_default()
            }
        })
        .min()
        .unwrap_or_default();

    for (key, &value) in &bake.recipe {
        if let Some(p) = bake.pantry.get_mut(key) {
            *p -= cookies * value;
        }
    }

    HttpResponse::Ok().json(serde_json::json!(
        {
            "cookies": cookies,
            "pantry": bake.pantry,
        }
    ))
}

#[cfg(test)]
mod tests {
    use actix_web::cookie::Cookie;
    use actix_web::{body, test, App};

    #[actix_web::test]
    async fn test_part_1() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let req = test::TestRequest::get()
            .uri("/7/decode")
            .cookie(
                Cookie::build("recipe", "eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==")
                    .finish(),
            )
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();
        let response = std::str::from_utf8(&bytes).unwrap();
        assert_eq!(response, r#"{"flour":100,"chocolate chips":20}"#);
    }

    #[actix_web::test]
    async fn test_part_2() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let req = test::TestRequest::get()
            .uri("/7/bake")
            .cookie(Cookie::build("recipe", "eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319").finish())
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();
        let response: super::BakeCookieResponse = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(response.cookies, 4);
        assert_eq!(response.pantry.flour, 5);
        assert_eq!(response.pantry.sugar, 307);
        assert_eq!(response.pantry.butter, 2002);
        assert_eq!(response.pantry.baking_powder, 825);
        assert_eq!(response.pantry.chocolate_chips, 257);
    }
}
