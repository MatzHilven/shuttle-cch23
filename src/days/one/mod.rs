use actix_web::{get, HttpResponse, Responder, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(part_1_and_2);
}

#[get("/1/{nums}*")]
async fn part_1_and_2(nums: web::Path<String>) -> impl Responder {
    let nums = nums.split('/').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let result = nums.iter().fold(0, |acc, &num| acc ^ num).pow(3);
    HttpResponse::Ok().body(format!("{}", result))
}



#[cfg(test)]
mod tests {
    use actix_web::{App, body, test};
    #[actix_web::test]
    async fn test_part_1() {
        let app = test::init_service(
            App::new().configure(super::configure)
        ).await;
        let req = test::TestRequest::get().uri("/1/4/8").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert_eq!(bytes.unwrap(), "1728");
    }

    #[actix_web::test]
    async fn test_part_2_1() {
        let app = test::init_service(
            App::new().configure(super::configure)
        ).await;
        let req = test::TestRequest::get().uri("/1/10").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert_eq!(bytes.unwrap(), "1000");
    }

    #[actix_web::test]
    async fn test_part_2_2() {
        let app = test::init_service(
            App::new().configure(super::configure)
        ).await;
        let req = test::TestRequest::get().uri("/1/4/5/8/10").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;
        assert_eq!(bytes.unwrap(), "27");
    }
}
