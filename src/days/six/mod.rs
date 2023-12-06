use actix_web::{HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};

pub fn configure(cfg: &mut web::ServiceConfig) {
    // cfg.service(part_1);
    cfg.service(part_2);
}

#[derive(Serialize, Deserialize)]
struct ElfResponse {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

// #[post("/6")]
// async fn part_1(string: String) -> impl Responder {
//     let count = string.matches("elf").count();
//
//     HttpResponse::Ok().json(ElfResponse { elf: count })
// }

#[post("/6")]
async fn part_2(string: String) -> impl Responder {
    let elf = string.matches("elf").count();
    let elf_on_a_shelf = string.matches("elf on a shelf").count();
    let shelf_with_no_elf_on_it = string.matches("shelf").count() - elf_on_a_shelf;

    HttpResponse::Ok().json(ElfResponse { elf, elf_on_a_shelf, shelf_with_no_elf_on_it })
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test, http::header::ContentType, body};

    #[actix_web::test]
    async fn test_part_1() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let body = "The mischievous elf peeked out from behind the toy workshop,
      and another elf joined in the festive dance.
      Look, there is also an elf on that shelf!";

        let req = test::TestRequest::post()
            .uri("/6")
            .insert_header(ContentType::plaintext())
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        let response: super::ElfResponse = serde_json::from_slice(&bytes.unwrap()).unwrap();

        assert_eq!(response.elf, 4);
    }

    #[actix_web::test]
    async fn test_part_2() {
        let app = test::init_service(App::new().configure(super::configure)).await;

        let body = "there is an elf on a shelf on an elf. there is also another shelf in Belfast.";

        let req = test::TestRequest::post()
            .uri("/6")
            .insert_header(ContentType::plaintext())
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        let response: super::ElfResponse = serde_json::from_slice(&bytes.unwrap()).unwrap();

        assert_eq!(response.elf, 5);
        assert_eq!(response.elf_on_a_shelf, 1);
        assert_eq!(response.shelf_with_no_elf_on_it, 1);
    }
}
